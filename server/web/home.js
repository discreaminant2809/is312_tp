const blogListElem = document.querySelector(`#blog-list`);
const searchBarKwElem = document.querySelector(`#search-bar-kw`);
const searchBarAuthorElem = document.querySelector(`#search-bar-author`);
const searchBarDateElem = document.querySelector(`#search-bar-date`);
const searchBarSubmitElem = document.querySelector(`#search-bar-submit`);

searchBarSubmitElem.onclick = async () => {
    blogListElem.innerHTML = ``;

    // const keyword = searchBarKwElem.value ? searchBarKwElem.value;
    // const author = searchBarAuthorElem.value ? searchBarAuthorElem.value : undefined;
    // const since = searchBarDateElem.value ? new Date(searchBarDateElem.value).getTime() : undefined;
    //
    const query = [
        `keyword=${searchBarKwElem.value}`,
        `author=${searchBarAuthorElem.value}`,
    ];
    if (searchBarDateElem.value) {
        query.push(`since=${new Date(searchBarDateElem.value).getTime()}`);
    }
    // debugger
    const res = await fetch(
        `./api/searchpost?${query.join(`&`)}`,
        {
            method: `GET`,
            mode: 'same-origin',
            headers: {
                'Content-Type': `application/json`,
            },
        }
    );

    if (!res.ok) {
        return;
    }

    const posts = await res.json();
    posts.forEach(post => {
        const articleElem = document.createElement(`article`);
        articleElem.classList.add(`blog-post`);
        {
            const postHeaderElem = document.createElement(`div`);
            postHeaderElem.classList.add(`post-header`);
            {
                const postTitleElem = document.createElement(`h2`);
                postTitleElem.classList.add(`post-title`);
                postTitleElem.textContent = post.title;
                postHeaderElem.appendChild(postTitleElem);
            }
            {
                const postDateElem = document.createElement(`p`);
                postDateElem.classList.add(`post-date`);
                postDateElem.textContent = `${new Date(post.dateNum).toDateString()}`;
                postHeaderElem.appendChild(postDateElem);
            }
            articleElem.appendChild(postHeaderElem);
        }
        {
            const postAuthorElem = document.createElement(`p`);
            postAuthorElem.classList.add(`post-author`);
            postAuthorElem.textContent = post.author;
            articleElem.appendChild(postAuthorElem);
        }
        {
            const postSummaryElem = document.createElement(`p`);
            postSummaryElem.classList.add(`post-summary`);
            postSummaryElem.textContent = deltaToSummary(post.content);
            articleElem.appendChild(postSummaryElem);
        }
        blogListElem.appendChild(articleElem);
    });
};

onkeydown = async e => {
    if (e.key === `Enter`) {
        await searchBarSubmitElem.onclick();
    }
}