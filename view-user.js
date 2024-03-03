const tabElems = document.querySelectorAll(`.tab`);
const changePwdElem = document.querySelector(`#change-pwd-text`);

const SELECTED_CLASS_NAME = `selected`;
tabElems.forEach((tabElem, _, tabElems) => {
    tabElem.onclick = () => {
        tabElems.forEach(tabElem => tabElem.classList.remove(SELECTED_CLASS_NAME));
        tabElem.classList.add(SELECTED_CLASS_NAME);
    };
});
tabElems[0].classList.add(SELECTED_CLASS_NAME);

