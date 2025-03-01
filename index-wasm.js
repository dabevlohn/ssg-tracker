import init, {
  create_token,
  validate_token,
  logparser,
} from "./pkg/ssg_tracker.js";
init().then(() => {
  const token = create_token();
  const claim = validate_token(token + "");
  const ltext = logparser("http://127.0.0.1:8000/log-sample.log");

  document.getElementById("tracker-log-1").textContent = token;
  document.getElementById("tracker-log-2").textContent = claim;
  ltext.then((res) => {
    document.getElementById("tracker-log-3").textContent = res;
  });
});
