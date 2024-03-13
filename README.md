# Kubernetes and Spin Experimentation

This repository contains a set of examples for how to run Spin applications on Kubernetes, using [SpinKube](https://spinkube.dev).

The repository contains all the you need to try the scenarios. Most sceanrios re-use scripts or resources form eachother, so there's a Makefile included to help run the scenarios.

You can either run the scenarios in total (including setitng up and tearing down a local cluster), or you can ru the individual tasks from each scenario.

## Scenarios

- [x] [Scenario 1 - Hello World on K3d](#scenario-1--hello-world-on-k3d)
- [x] [Scenario 2 - Using KV on K3d](#scenario-2---using-kv-on-k3d)
- [x] [Scenario 3 - Using sqlite on K3d](#scenario-3---using-sqlite-on-k3d)
- [x] [Scenario 4 - K3d, RabbitMQ, Dapr, and a queue consumer app](#scenario-4---k3d-rabbitmq-dapr-and-a-consumer-app)

### Directory layout

- [`apps`](/apps/) - The source for all apps used in the scenarios
- [`cluster_config`](/cluster_config/) - Configurations to apply to the K8s cluster
- [`dapr`](/dapr/) - Dapr configurations (bindings)
- [`deployments`](/deployments/) - K8s deployment specs

## Scenario 1 - Hello World on K3d

1. Creates a local k3d cluster with SpinKube
2. Install a [Hello World app](./apps/rust-hello/) using SpinKube

```shell
make scenario_1
```

## Scenario 2 - Using KV on K3d

1. Create a local k3d cluster with the Spin shim and runtime classes.
2. Install Redis, and deploy an application using Redis as the backing store for wasi-keyvalue.

```shell
make k3d_scenario_2
```

## Scenario 3 - Using Sqlite on K3d

1. Create a local k3d cluster with the Spin shim and runtime classes.
2. Install sqld, and deploy an application using sqlite as the backing store for wasi-sqlite.

```shell
make k3d_scenario_3
```

## Scenario 4 - K3d, RabbitMQ, Dapr, and a consumer app

Note: The container image used to produce messags fr the queue currently only support aarch (Mac M). You can produce messages for the qeue directly from the RabbitMS console as an alternative.

1. Create a local k3d cluster with the Spin shim and runtime classes.
2. Deploy RabbitMQ and Dapr
3. Deploy consumer app and Dapr bindings
4. Manually - Run the Producer app to app messages to the queue

```shell
make k3d_scenario_4

To see the rabbit queue:
kubectl run producer -it --rm --image=ghcr.io/mikkelhegn/producer rabbit-producer
```

