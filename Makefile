# Variables - NOT ALL ARE IMPLEMENTED
IMG_REPO := ttl.sh
CLUSTER_NAME := spin-k8s
AGENTS := 0
AARCH := true
CONTAINERD_SHIM_SPIN_VERSION := v0.14.0

# Bending the rules of make as a tool
.PHONY: *

# Scenarios to show various Spin on Kubernetes features and use-cases

## Hello World app deployed in k3d
scenario_1: create_k3d_cluster deploy_spin_operator build_hw_app deploy_hw_app

## App using wasi-keyvalue deployed in k3d, backed by redis
scenario_2: create_k3d_cluster deploy_spin_operator deploy_redis build_kv_app deploy_kv_app

## App using wasi-sqlite deployed in k3d, backed by sqld
scenario_3: create_k3d_cluster deploy_spin_operator deploy_sqld build_sql_app deploy_sql_app

## App consuming messages from a RabbitMQ queue. Includes Rabbit, Dapr, a consumer app and an app to produce messages
scenario_4: create_k3d_cluster deploy_spin_operator deploy_rabbitmq deploy-dapr consumer_app_bpd rabbit_producer_bp
	$(info Run `kubectl logs -l app=rabbit-consumer -f` to follow logs from the consumer.)
	$(info In another shell, run `make rabbit_producer_run` to add messages to the queue.)


# Individual tasks, which can be re-used across scenarios

## Cluster tasks
### Deletes and creates a k3d cluster using containerd-shim image
create_k3d_cluster:
	k3d cluster delete $(CLUSTER_NAME)
	k3d cluster create $(CLUSTER_NAME) \
		--image ghcr.io/spinkube/containerd-shim-spin/k3d:$(CONTAINERD_SHIM_SPIN_VERSION) \
		-p "8081:80@loadbalancer" \
		--servers-memory 10G \
		--agents $(AGENTS)

### Deploy Spin Operator
deploy_spin_operator: deploy_cert_manager 
	kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.crds.yaml
	kubectl apply -f https://github.com/spinkube/spin-operator/releases/download/v0.1.0/spin-operator.runtime-class.yaml

	helm install spin-operator --namespace spin-operator --create-namespace --version 0.1.0 --wait oci://ghcr.io/spinkube/charts/spin-operator
	
	kubectl apply -f https://raw.githubusercontent.com/spinkube/spin-operator/v0.1.0/config/samples/spin-shim-executor.yaml
	kubectl apply -f https://raw.githubusercontent.com/spinkube/spin-operator/v0.1.0/config/samples/spin-runtime-class.yaml

### Deploy Cert Manager
deploy_cert_manager:
	kubectl apply -f https://github.com/cert-manager/cert-manager/releases/download/v1.14.3/cert-manager.crds.yaml
	helm repo add jetstack https://charts.jetstack.io
	helm repo update
	helm install cert-manager jetstack/cert-manager --namespace cert-manager --create-namespace --version v1.14.3 --wait


## Apps
### Build Hello World app
build_hw_app:
	spin build -f apps/rust-hello/spin.toml
	spin registry push -f apps/rust-hello/spin.toml $(IMG_REPO)/rust-hello:1h

### Deploy Hello World app
deploy_hw_app:
	cat deployments/rust_hello.yaml | sed "s,{{IMG_REPO}},$(IMG_REPO)/rust-hello:1h," | kubectl apply -f -
	$(info run `kubectl port-forward svc/rust_hello 8080:80` and `curl -i localhost:8080`)

# Build app using wasi-keyvalue
build_kv_app:
	spin build -f apps/kv/spin.toml
	spin registry push -f apps/kv/spin.toml $(IMG_REPO)/spin-kv:1h

# Deploy apps using wasi-keyvalue
deploy_kv_app:
	cat deployments/kv.yaml | sed "s,{{IMG_REPO}},$(IMG_REPO)/spin-kv:1h," | kubectl apply -f -
	$(info run `kubectl port-forward svc/kv 8080:80` and `curl -i localhost:8080`)

# Deploy a simple Redis instance
deploy_redis:
	kubectl delete -f deployments/redis.yaml --ignore-not-found
	kubectl apply -f deployments/redis.yaml

# Deploy a simple sqld instance
deploy_sqld:
	kubectl delete -f deployments/sqld.yaml --ignore-not-found
ifeq ($(AARCH), true)
	kubectl apply -f deployments/sqld.yaml
else
	cat deployments/sqld.yaml | sed "s,ghcr.io/mikkelhegn/sqld:latest,ghcr.io/tursodatabase/libsql-server:latest," | kubectl apply -f -
endif

# Build app using wasi-sqlite
build_sql_app:
	spin build -f apps/sql/spin.toml
	spin registry push -f apps/sql/spin.toml $(IMG_REPO)/sql-app:1h

# Deploy app using wasi-sqlite
deploy_sql_app:
	cat deployments/sql.yaml | sed "s,{{IMG_REPO}},$(IMG_REPO)/sql-app:1h," | kubectl apply -f -
	$(info run `kubectl port-forward svc/kv 8080:80` and `curl -i localhost:8080`)

# Deploy RabbitMQ
# Can browse to localhost:15672 guest/guest after enabling a port-forward to the pod
deploy_rabbitmq:
	kubectl delete -f deployments/rabbitmq.yaml --ignore-not-found
	kubectl apply -f deployments/rabbitmq.yaml

# App to consume messages from a queue in RabbitMQ
consumer_app_bpd: build_consumer_app deploy_consumer_app

build_consumer_app:
	spin build -f apps/rabbit-consumer/spin.toml
	spin registry push -f apps/rabbit-consumer/spin.toml $(IMG_REPO)/rabbit-consumer:1h

deploy_consumer_app:
	helm uninstall rabbit-shared-instance --ignore-not-found
	helm install rabbit-shared-instance oci://registry-1.docker.io/daprio/dapr-shared-chart --set shared.appId=rabbit-consumer --set shared.remoteURL=rabbit-consumer --set shared.remotePort=80
	kubectl delete -f dapr/bindings/rabbitmq_binding.yaml --ignore-not-found
	cat deployments/rabbit_consumer.yaml | sed "s,{{IMG_REPO}},$(IMG_REPO)/rabbit-consumer:1h," | kubectl apply -f -
	kubectl apply -f dapr/bindings/rabbitmq_binding.yaml

# Deploy Dapr
deploy-dapr:
	helm uninstall dapr --namespace dapr-system --ignore-not-found
	helm repo add dapr https://dapr.github.io/helm-charts/
	helm upgrade --install dapr dapr/dapr \
		--version=1.12 \
		--namespace dapr-system \
		--create-namespace \
		--wait
	helm install dapr-dashboard dapr/dapr-dashboard --namespace dapr-system
	kubectl get pods --namespace dapr-system

# Build and push an app to produce messages to a Rabbit queue
rabbit_producer_bp:
	cross build --manifest-path=apps/rabbit-producer/Cargo.toml --target aarch64-unknown-linux-musl --release
	docker build -t $(IMG_REPO)/producer:latest apps/rabbit-producer
	docker push $(IMG_REPO)/producer:latest
#TODO use image load

# Run the app to produce 100 messages to a Rabbit queue
rabbit_producer_run:
	kubectl run producer -i --rm --image=$(IMG_REPO)/producer --restart=Never --command -- ./rabbit-producer --queue test --server amqp://rabbitmq --messages 100


### For future work

# 	kubectl port-forward --namespace monitoring svc/kube-stack-prometheus-grafana 8080:80
#	kubectl port-forward --namespace monitoring svc/kube-stack-prometheus-kube-prometheus 9090:9090
prom_install:
	kubectl create namespace monitoring
	helm repo add prometheus-community https://prometheus-community.github.io/helm-charts
	helm repo update
	helm upgrade --namespace monitoring --install kube-stack-prometheus prometheus-community/kube-prometheus-stack --set prometheus-node-exporter.hostRootFsMount.enabled=false
	kubectl get secret --namespace monitoring kube-stack-prometheus-grafana -o jsonpath='{.data.admin-user}' | base64 -d
	kubectl get secret --namespace monitoring kube-stack-prometheus-grafana -o jsonpath='{.data.admin-password}' | base64 -d

