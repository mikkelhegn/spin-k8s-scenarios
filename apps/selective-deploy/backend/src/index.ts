import { Kv, ResponseBuilder, Router } from "@fermyon/spin-sdk";
// We have to ignore here because `spin deps` does not yet generate type bindings for TS
// @ts-ignore
import { grayscale, sepia, } from "component:image-manipulation-lib/image-manipulation";

let router = Router();

router.post("/images/grayscale", async (_meta, req, res) => {
  console.log("[backend]: Got grayscale transform request");
  let img = await req.arrayBuffer();
  let transformed = await transform("grayscale", img);

  res.set({ "Content-Type": "image/jpeg" });
  res.send(transformed);
});

router.post("/images/sepia", async (_meta, req, res) => {
  console.log("[backend]: Got sepia transform request");
  let img = await req.arrayBuffer();
  let transformed = await transform("sepia", img);

  res.set({ "Content-Type": "image/jpeg" });
  res.send(transformed);
});

async function transform(t: string, img: ArrayBuffer): Promise<Uint8Array> {
  let contentDigest = await digest(img);
  let kvKey = `${t}-${contentDigest}`;
  console.log("[backend]: Content digest of input image: " + contentDigest);
  let kv = Kv.openDefault();

  let transformed;
  let cache = kv.get(kvKey);
  if (cache) {
    console.log("[backend]: Found transformed image in cache");
    transformed = cache;
  } else {
    switch (t) {
      case "grayscale":
        transformed = grayscale(new Uint8Array(img), 100);
        break;
      case "sepia":
        transformed = sepia(new Uint8Array(img), 100);
        break;
      default:
        throw new Error("[backend]: Unknown image transform");
    }

    kv.set(kvKey, transformed);
  }

  console.log("[backend]: Returning transformed image");

  return transformed;
}

async function digest(buf: ArrayBuffer): Promise<string> {
  let hashBuffer = await crypto.subtle.digest("sha-256", buf);
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  const hashHex = hashArray
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");

  return hashHex.toUpperCase();
}

export async function handler(req: Request, res: ResponseBuilder) {
  await router.handleRequest(req, res);
}
