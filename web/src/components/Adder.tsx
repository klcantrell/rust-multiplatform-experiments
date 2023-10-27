import { useEffect } from "react";
import initShared, { sayAfter } from "shared";

export default function Adder() {
  useEffect(() => {
    initShared("shared_klc_bg.wasm").then(() => {
      sayAfter(BigInt(2000), "Kalalau").then((result) => {
        console.log(result);
      });
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
