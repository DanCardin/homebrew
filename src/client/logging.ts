export function log(message: string, level?: "info" | "warn" | "error") {
  if (process.env.NODE_ENV !== "production") {
    if (level === "error") {
      console.error(message);
    } else if (level === "warn") {
      console.warn(message);
    } else {
      console.log(message);
    }
  }
}

export default {
  log: log,
  debug: (message: string) => log(message, "debug"),
  info: (message: string) => log(message, "info"),
  warning: (message: string) => log(message, "warning"),
  error: (message: string) => log(message, "error"),
};
