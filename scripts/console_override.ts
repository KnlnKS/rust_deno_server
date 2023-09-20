((globalThis) => {
  const originalConsole = console;
  const consoleProxy = new Proxy(originalConsole, {
    get(_target, propKey) {
      // deno-lint-ignore no-explicit-any
      return function (...args: any[]) {
        // TODO: Check if an object and stringify if possible?
        Deno?.core?.print(JSON.stringify({ function: propKey, args }) + ",");
      };
    },
  });
  globalThis.console = consoleProxy;
})(globalThis);
