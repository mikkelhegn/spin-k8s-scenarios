import { Router, ResponseBuilder } from "@fermyon/spin-sdk";
import * as Handlebars from "handlebars";

let router = Router();

router.get("/", (_, req, res) => {
  console.log("[frontend]: Handling /");
  defaultRoute(req, res);
});
router.get("/favicon.ico", (_, req, res) => {
  console.log("[frontend]: Handling /favicon.ico");
  faviconRoute(req, res);
});
router.all("/api/*", (_, req, res) => {
  console.log("[frontend]: Handling API");
  handleApi(req, res);
});

async function defaultRoute(_req: Request, res: ResponseBuilder) {
  const template = Handlebars.compile(imageManipulation);
  const response = template({ serviceAddress: getBackendAdd() });

  res.set({ "content-type": "text/html" });
  res.send(response);
}

async function handleApi(req: Request, res: ResponseBuilder) {
  let transform = req.url.substring(req.url.lastIndexOf("/"));
  console.log(`[frontend]: Transform: ${transform}`);
  const response = await fetch(
    //`http://backend.spin.internal/images${transform}`,
    `/images${transform}`,
    {
      method: req.method,
      headers: { "content-type": "application/octet-stream" },
      body: req.body,
    },
  );

  if (!response.ok)
    throw new Error("[frontend]: Request returned a bad response code");

  const data = await response.arrayBuffer();

  console.log("[frontend]: Got transformed image");

  res.statusCode = response.status;
  res.headers = response.headers;

  console.log("[frontend]: Sending back transformed image");
  res.send(data);
}

export async function handler(req: Request, res: ResponseBuilder) {
  await router.handleRequest(req, res);
}

const imageManipulation: String = `
<!DOCTYPE html>
<html lang="en">

<head>
	<meta charset="UTF-8">
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
	<title>Image Transformations powered by Spin</title>
	<link href="https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" rel="stylesheet">
</head>

<body class="bg-gray-100 font-sans antialiased">
	<header class="bg-indigo-600 py-6 shadow-lg">
		<h1 class="text-center text-3xl font-bold text-white">Image Transformations powered by Spin and Wasm components
		</h1>
	</header>
	<main class="max-w-md mx-auto mt-10 p-6 bg-white rounded-lg shadow-md">
		<div class="mb-4">
			<label for="filter" class="block text-gray-700 font-semibold mb-2">Choose an image transformation</label>
			<select id="filter"
				class="w-full px-4 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-indigo-500">
				<option value="{{ serviceAddress }}/api/grayscale">Grayscale</option>
				<option value="{{ serviceAddress }}/api/sepia">Sepia</option>
			</select>
		</div>
		<div class="mb-6">
			<label for="image" class="block text-gray-700 font-semibold mb-2">Upload an image:</label>
			<input type="file" id="image" accept="image/*"
				class="w-full px-4 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-indigo-500">
		</div>
		<button onclick="transform()"
			class="w-full bg-indigo-600 text-white font-bold py-2 rounded hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500">
			Transform Image
		</button>
	</main>
	<div style="display: flex">
		<section id="original" class="w-full max-w-screen-lg mx-auto mt-6 p-4 text-center"
			style="padding-left: 5%; padding-right: 5%;"></section>
		<section id="result" class="w-full max-w-screen-lg mx-auto mt-6 p-4 text-center"
			style="padding-left: 5%; padding-right: 5%;"></section>
	</div>
	<script type="text/javascript">
		async function transform() {
			const filter = document.getElementById("filter").value;
			const fileInput = document.getElementById("image");
			const file = fileInput.files[0];
			const original = document.getElementById("original");
			const transformed = document.getElementById("result");
			if (!file) {
				console.error("No file selected");
				return false;
			}

			const ab = await file.arrayBuffer();
			const ba = new Uint8Array(ab);
			try {
				const response = await fetch(filter, {
					method: "POST",
					headers: {
						"content-type": "application/octet-stream",
					},
					body: ba
				});
				if (!response.ok) throw new Error("Request returned a bad response code");

				const transformedImageBlob = await response.blob();
				appendImage(original, new Blob([ab], {type: file.type}), "Original Image");
				appendImage(transformed, transformedImageBlob, "Transformed Image");
			} catch (err) {
				console.error(err);
			} finally {
				return false;
			}
		}

		function appendImage(targetElement, imageBlob, caption) {
			const url = URL.createObjectURL(imageBlob);
			targetElement.innerHTML = '';
			const img = document.createElement("img");
			img.src = url;
			img.alt = caption;
			img.style.maxWidth = "100%";
			const header = document.createElement("h2");
			header.innerText = caption;
			header.className = "text-xl text-center"
			targetElement.appendChild(header);
			targetElement.appendChild(img);
		}
	</script>
</body>

</html>
`;
function getBackendAdd(): String {
  return "";
}

function faviconRoute(_req: Request, res: ResponseBuilder) {
  res.status(200).send();
}
