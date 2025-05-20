function initToc(title, list)
{
    title.classList.toggle('expanded', false);
    list.classList.toggle('expanded', false);
    list.style.display = 'block';
    list.style.maxHeight = '0';
    list.style.overflow = 'hidden';
    list.style.transition = 'max-height 0.3s ease-out';
}

document.addEventListener('DOMContentLoaded', () => {
    const tocTitle = document.querySelector('.toc-title');
    const tocList = document.querySelector('.toc-list');
    
    if (tocTitle && tocList) {
        initToc(tocTitle, tocList);
        const toggleToC = () => {
            tocTitle.classList.toggle('expanded');
            tocList.classList.toggle('expanded');
            const expanded = tocList.classList.contains('expanded');
            tocList.style.maxHeight = expanded ? `${tocList.scrollHeight}px` : '0';
        };

        tocTitle.addEventListener('click', toggleToC);
    }
});

