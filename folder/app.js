// Grab DOM elements
const nameInput = document.getElementById('name');
const commentInput = document.getElementById('comment');
const submitBtn = document.getElementById('submitBtn');
const commentList = document.getElementById('commentList');

// Function to load and display comments
async function loadComments() {
    const res = await fetch('/api/visitors');
    const comments = await res.json();

    commentList.innerHTML = ''; // clear current list
    comments.forEach(c => {
        const li = document.createElement('li');
        li.textContent = `${c.name}: ${c.comment}`;
        commentList.appendChild(li);
    });
}

// Event listener for submitting new comments
submitBtn.addEventListener('click', async () => {
    const name = nameInput.value.trim();
    const comment = commentInput.value.trim();
    if (!name || !comment) return; // simple validation

    await fetch('/api/visitors', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, comment }),
    });

    // Clear input fields
    nameInput.value = '';
    commentInput.value = '';

    // Reload comments to show the new one
    loadComments();
});

// Load comments when the page loads
loadComments();

