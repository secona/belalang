import './main.css'
import * as rs from "rs";

const title = document.getElementById("title");

if (title) {
  title.innerHTML = rs.message();
}
