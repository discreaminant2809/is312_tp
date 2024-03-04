"use strict";

const tabElems = document.querySelectorAll(`.tab`);
const tabContentElems = document.querySelectorAll(`.tab-content`);
const changePwdTextElem = document.querySelector(`#change-pwd-text`);
const changePwdForm = document.querySelector(`#change-pwd-form`);
const changePwdReenterElem = document.querySelector(`#change-pwd-reenter`);
const changePwdNewPwd = document.querySelector(`#change-pwd-new-pwd`);
const changePwdCancelElem = document.querySelector(`#change-pwd-cancel`);

const SELECTED_CLASS_NAME = `selected`;
tabElems.forEach((tabElem, i, tabElems) => {
    tabElem.onclick = () => {
        tabElems.forEach(tabElem => tabElem.classList.remove(SELECTED_CLASS_NAME));
        tabContentElems.forEach(tabContentElem => tabContentElem.hidden = true);
        tabElem.classList.add(SELECTED_CLASS_NAME);
        tabContentElems[i].hidden = false;
    };
});
tabElems[0].classList.add(SELECTED_CLASS_NAME);
tabContentElems[0].hidden = false;

changePwdTextElem.onclick = () => {
    changePwdForm.hidden = false;
};

changePwdCancelElem.onclick = e => {
    e.preventDefault();

    changePwdForm.hidden = true;
    changePwdReenterElem.value = ``;
    changePwdNewPwd.value = ``;
};