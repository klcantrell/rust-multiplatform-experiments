import { useEffect } from "react";
import initShared from "shared";

export default function Adder() {
  useEffect(() => {
    initShared("shared_klc_bg.wasm").then(({ add_stuff }) => {
      console.log(`4 + 5 = ${add_stuff(4, 5)}`);
    });
  }, []);

  return (
    <div>
      <header>
        <p>Adder</p>
      </header>
    </div>
  );
}
