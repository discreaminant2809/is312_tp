const toolbarOptions = [
    [{'font': []}],
    ['bold', 'italic', 'underline', 'strike'],
    ['blockquote', 'code-block'],
    ['link', 'image'],

    [{'header': 1}, {'header': 2}],
    [{'list': 'ordered'}, {'list': 'bullet'}],
    [{'script': 'sub'}, {'script': 'super'}],
    [{'indent': '+1'}, {'indent': '-1'}],
    [{'direction': 'rtl'}],

    [{'size': ['small', false, 'large', 'huge']}],
    [{'header': [1, 2, 3, false]}],

    [{'color': []}, {'background': []}],
    [{'align': []}],

    ['clean'] // clear-all-formats button
];

const postContentEditor = new Quill('#post-content', {
    modules: {
        toolbar: toolbarOptions
    },
    theme: 'snow'
});