import init, { create_token, validate_token } from "./pkg/ssg_tracker.js";
init().then(() => {
  const current_location_origin = window.location.origin;
  const current_location_path = window.location.pathname;
  const token = create_token(
    current_location_origin,
    current_location_path,
    "my@ema.il",
    "87987987088708",
  );
  const sub = validate_token(token + "");
  document.getElementById("tracker-log-1").textContent = token;
  document.getElementById("tracker-log-2").textContent = sub;
});
