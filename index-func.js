function getXemailHeader() {
  var header = "-000-";
  let req = new XMLHttpRequest();
  req.open("GET", window.location.href, true);
  req.send(null);
  req.onload = () => {
    header = req.getResponseHeader("content-type");
    console.log("Response Header:\n", header);
  };
  return header;
}
function getCurrentOrigin() {
  var origin = window.location.origin;
  if (origin == "null") {
    origin = window.location.protocol + "//" + window.location.host;
  } else {
    origin += "/";
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
