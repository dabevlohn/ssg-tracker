function trackUser(url) {
  let req = new XMLHttpRequest();
  req.open("GET", url, true);
  req.send(null);
  req.onload = () => {
    let header = req.getResponseHeader("content-type");
    console.log("Response Header:\n", header);
  };
}
function getCurrentOrigin() {
  var origin = window.location.origin;
  if (origin == "null") {
    origin = window.location.protocol + "//" + window.location.host;
  }
  return origin;
}
function getCurrentPathname() {
  var pathname = window.location.pathname;
  if (pathname == "/") {
    pathname += "index.html";
  }
  return pathname;
}
