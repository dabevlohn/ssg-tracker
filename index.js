import init, { create_token } from "./pkg/ssg_tracker.js";
init().then(() => {
  const current_location_origin = window.location.origin;
  const current_location_path = window.location.pathname;
  create_token(
    current_location_origin,
    current_location_path,
    "my@ema.il",
    "87987987088708",
  );
});
