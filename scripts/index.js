import init from "../pkg/survey_project_checker.js"

window.onload = () => {
    // disable mousewheel scroll on number input
    document.addEventListener("wheel", function(event){
        if(document.activeElement.type === "number"){
            document.activeElement.blur();
        }
    });

    init();
};