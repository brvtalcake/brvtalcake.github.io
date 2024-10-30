
function get_body_elem_css_color(elemtag, elemclass, index)
{
    var tagged = document.getElementsByTagName(elemtag);

    var satisfying = Array.from(tagged).filter(elem => elem.className === elemclass);

    if (satisfying.length < 1)
    {
        console.error("Could not find any elements with tag " + elemtag + " and class " + elemclass);
        return null;
    }

    var elem = satisfying[index ? index : 0];
    var style = window.getComputedStyle(elem);
    return style.color;
}

function log_elem_color(elemtag, elemclass)
{
    var color = get_body_elem_css_color(elemtag, elemclass);
    console.log(color);
}

// Print elem color when clicked
function log_color_on_click(elemtag, elemclass)
{
    var elems = document.getElementsByTagName(elemtag);

    var satisfying = Array.from(elems).filter(elem => elem.className === elemclass);

    satisfying.forEach(elem => {
        elem.addEventListener("click", function() {
            var style = window.getComputedStyle(elem);
            console.log(style.color);
        });
    });
}

log_color_on_click("h1", "title");