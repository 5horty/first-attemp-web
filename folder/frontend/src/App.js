import "./App.css"; 
import image from "./catsuars.jpg";
import {useState, useEffect} from "react";

export default function Comments(){
  const [name, setName] =  useState("");
  const [comment, setComment] =  useState("");
  const [comments, setComments] =  useState([]);


useEffect(()=>{
  loadComments();
},
[]);
// Function to load and display comments
async function loadComments() {
    const res = await fetch("/api/:)");
    const data = await res.json();
    setComments(data);
}
async function handleSubmit(e){
  e.preventDefault();
  if (!name.trim() || !comment.trim()) return;

  await fetch("api/:)",{
    method: "POST",
    headers : {"Content-Type": "application/json"},
    body : JSON.stringify({name, comment}),
  });
  setName("");
  setComment("");
  loadComments();
}


return (
    <div className="container">
      <div>
        <h1>Hello Everyone</h1>
      </div>
      <article>
        <p>
        testing testing





        
        </p>
        <img src = {image} alt = "dino cat"/>
      </article>
      <h2>Comments</h2>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Your name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <input
          type="text"
          placeholder="Your comment"
          value={comment}
          onChange={(e) => setComment(e.target.value)}
        />
        <button type="submit">Submit</button>
      </form>

      <ul>
        {comments.map((c, idx) => (
          <li key={idx}>
            {c.name}: {c.comment}
          </li>
        ))}
      </ul>
    </div>
  );
}


