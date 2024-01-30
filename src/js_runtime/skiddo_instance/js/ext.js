((globalThis) => {
  const core = Deno.core;
  const skiddo_file = core.ops.op_get_skiddo_file();
  const op_log = core.ops.op_log;
  function argsToMessage(...args) {
    const seen = new WeakSet();
    return args
      .map((arg) =>
        JSON.stringify(arg, (_k, v) => {
          if (v === globalThis) {
            return "[Not Allowed]";
          }
          if (typeof value === "object" && value !== null) {
            if (seen.has(v) || v === arg) {
              return "[Circular]";
            }
            seen.add(v);
          }
          return typeof v == "function" ? v.toString() : v;
        })
      )
      .join(" ");
  }
  // Populate global context with the console API
  Object.defineProperty(globalThis, "console", {
    value: {
      log: (...args) => {
        op_log("Info", argsToMessage(...args));
      },
      error: (...args) => {
        op_log("Error", argsToMessage(...args));
      },
    },
  });
  let timers = {
    setTimeout: function (callback, delay) {
      return core.queueTimer(core.getTimerDepth() + 1, false, delay, callback);
    },
    setInterval: function (callback, delay) {
      return core.queueTimer(core.getTimerDepth() + 1, true, delay, callback);
    },
    clearTimeout: function (id) {
      core.cancelTimer(id);
    },
    clearInterval: function (id) {
      core.cancelTimer(id);
    },
    unrefTimer: function (id) {
      core.unrefTimer(id);
    },
    refTimer: function (id) {
      core.refTimer(id);
    },
  };

  Object.keys(timers).forEach((key) => {
    globalThis[key] = timers[key];
  });

  class Skiddo {
    skiddo_file;
    constructor(skiddo_file) {
      this.skiddo_file = skiddo_file;
    }
  }

  Object.defineProperty(globalThis, "skiddo", {
    value: new Skiddo(skiddo_file),
    writable: false,
    configurable: false,
    enumerable: true,
  });
  globalThis.console.log(globalThis.skiddo);
})(globalThis);

// Sandbox :3
delete Deno.core;
delete Deno.__op__;
