import init, { create_token, validate_token } from "./pkg/ssg_tracker.js";
init().then(() => {
  const token = create_token();
  const claim = validate_token(token + "");
  document.getElementById("tracker-log-1").textContent = token;
  document.getElementById("tracker-log-2").textContent = claim;
});
