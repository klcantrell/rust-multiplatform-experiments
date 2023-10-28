import { useEffect } from "react";
import initShared, { getIp } from "shared";

export default function Adder() {
  useEffect(() => {
    initShared("shared_klc_bg.wasm").then(() => {
      getIp().then(console.log);
    });
  }, []);

  return (
    <div>
      <header>
        <p>IP fetcher</p>
      </header>
    </div>
  );
}
