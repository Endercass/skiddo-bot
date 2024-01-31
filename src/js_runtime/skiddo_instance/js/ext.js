((globalThis) => {
  const core = Deno.core;

  const op_init_state = core.ops.op_init_state;

  const skiddo_file = core.ops.op_get_skiddo_file();
  const op_fs_create_file = core.ops.op_fs_create_file;
  const op_fs_read_file = core.ops.op_fs_read_file;
  const op_fs_write_file = core.ops.op_fs_write_file;
  const op_fs_remove_file = core.ops.op_fs_read_dir;
  const op_fs_create_dir = core.ops.op_fs_create_dir;
  const op_fs_read_dir = core.ops.op_fs_read_dir;
  const op_fs_remove_dir = core.ops.op_fs_remove_dir;
  const op_fs_stat = core.ops.op_fs_stat;
  const op_fs_copy = core.ops.op_fs_copy;
  const op_fs_rename = core.ops.op_fs_rename;

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

  const rust_vec_to_buffer = (vec) => {
    // Vec is object with as many keys as elements.
    // The keys are string representations of numbers.
    // The values are the elements.
    const array = new Array(vec.length);
    for (const key in vec) {
      array[Number(key)] = vec[key];
    }
    return array;
  };

  // Polyfill TextEncoder
  class TextEncoder {
    constructor(encoding = "utf-8") {
      if (encoding.toLocaleLowerCase() !== "utf-8") {
        throw new Error(`Unsupported encoding: ${encoding}`);
      }
    }

    encode(str) {
      // Try our best to allocate a buffer that is big enough.
      // UTF8 is a variable width encoding, however, so we may over-allocate.
      let utf8Bytes = new Array(str.length * 4);
      for (let i = 0; i < str.length; i++) {
        let code = str.charCodeAt(i);
        if (code < 128) {
          utf8Bytes.push(code);
        } else if (code < 2048) {
          utf8Bytes.push((code >> 6) | 192);
          utf8Bytes.push((code & 63) | 128);
        } else if (code < 65536) {
          utf8Bytes.push((code >> 12) | 224);
          utf8Bytes.push(((code >> 6) & 63) | 128);
          utf8Bytes.push((code & 63) | 128);
        } else {
          utf8Bytes.push((code >> 18) | 240);
          utf8Bytes.push(((code >> 12) & 63) | 128);
          utf8Bytes.push(((code >> 6) & 63) | 128);
          utf8Bytes.push((code & 63) | 128);
        }
      }
      // Trim off any over-allocation.
      utf8Bytes = utf8Bytes.flat();
      return new Uint8Array(utf8Bytes);
    }
  }

  // Polyfill TextDecoder
  class TextDecoder {
    constructor(encoding = "utf-8") {
      if (encoding.toLocaleLowerCase() !== "utf-8") {
        throw new Error(`Unsupported encoding: ${encoding}`);
      }
    }

    decode(data) {
      let result = "";
      let i = 0;

      while (i < data.length) {
        let byte = data[i];

        if (byte < 128) {
          result += String.fromCharCode(byte);
          i++;
        } else if (byte < 192) {
          throw new Error("Invalid UTF-8 sequence");
        } else if (byte < 224) {
          result += String.fromCharCode(
            ((byte & 31) << 6) | (data[i + 1] & 63)
          );
          i += 2;
        } else if (byte < 240) {
          result += String.fromCharCode(
            ((byte & 15) << 12) | ((data[i + 1] & 63) << 6) | (data[i + 2] & 63)
          );
          i += 3;
        } else {
          result += String.fromCharCode(
            ((byte & 7) << 18) |
              ((data[i + 1] & 63) << 12) |
              ((data[i + 2] & 63) << 6) |
              (data[i + 3] & 63)
          );
          i += 4;
        }
      }

      return result;
    }
  }

  // Populate global context with the filesystem API
  Object.defineProperty(globalThis, "fs", {
    value: {
      createFileSync: (path) => {
        return op_fs_create_file(path);
      },
      readFileSync: (path) => {
        return rust_vec_to_buffer(op_fs_read_file(path));
      },
      writeFileSync: (path, data) => {
        if (typeof data === "string") {
          data = new TextEncoder().encode(data);
        }
        return op_fs_write_file(path, data);
      },
      rmSync: (path) => {
        return op_fs_remove_file(path);
      },
      createDirSync: (path) => {
        return op_fs_create_dir(path);
      },
      readDirSync: (path) => {
        return op_fs_read_dir(path).split("\n");
      },
      rmdirSync: (path) => {
        return op_fs_remove_dir(path);
      },
      statSync: (path) => {
        return op_fs_stat(path);
      },
      copySync: (from, to) => {
        return op_fs_copy(from, to);
      },
      renameSync: (from, to) => {
        return op_fs_rename(from, to);
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

  Object.defineProperty(globalThis, "TextEncoder", {
    value: TextEncoder,
    writable: false,
    configurable: false,
    enumerable: true,
  });

  Object.defineProperty(globalThis, "TextDecoder", {
    value: TextDecoder,
    writable: false,
    configurable: false,
    enumerable: true,
  });

  op_init_state();

  globalThis.console.log("Hello world!");

  globalThis.console.log(globalThis.skiddo);

  globalThis.console.log(globalThis.fs);

  globalThis.console.log(
    new TextDecoder().decode(globalThis.fs.readFileSync("/test.txt"))
  );

  globalThis.fs.writeFileSync(
    "/test2.txt",
    "Hello" + " " + "\u{1F496}".repeat(5) + " " + "world!"
  );

  globalThis.console.log(
    new TextDecoder().decode(globalThis.fs.readFileSync("/test2.txt"))
  );
})(globalThis);

// Sandbox :3
delete Deno.core;
delete Deno.__op__;
