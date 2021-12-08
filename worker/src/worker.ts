import WASI from "wasi/wasi";
import module from "./wimg.wasm";
import { decode, encode, resize, WImg } from "wimg/wimg";

export default {
  async fetch(_request: Request) {
    const res = await fetch(
      "https://images.pexels.com/photos/416682/pexels-photo-416682.jpeg?w=2000"
    );

    console.time("transform");

    const wimg = await WASI.instantiate<WImg>(module);
    const [ptr, imgWidth, imgHeight] = await decode(
      wimg,
      await res.arrayBuffer()
    );
    console.log("size", imgWidth, imgHeight);

    const image = await encode(wimg, await resize(wimg, ptr, 500, 333));

    console.timeEnd("transform");

    return new Response(image.asUint8Array(), {
      headers: {
        "Content-Type": "image/jpeg",
      },
    });
  },
};
