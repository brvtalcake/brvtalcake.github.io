function isNoteExpanded(elem)
{
  if (elem.classList.contains("note-container"))
  {
    return isNoteExpanded(elem.querySelector(".note-toggle")) && isNoteExpanded(elem.querySelector(".note-content"));
  }
  else if (elem.classList.contains("note-toggle"))
  {
    return elem.getAttribute("aria-expanded") === "true";
  }
  else if (elem.classList.contains("note-content"))
  {
    return elem.style.maxHeight[0] !== "0";
  }
  return false;
}

function toggleNoteContent(noteContent)
{
  console.assert(noteContent !== null, "noteContent is null");
  console.assert(noteContent.classList.contains("note-content"), "noteContent is not a note-content");

  var expanded = isNoteExpanded(noteContent);
  if (expanded)
  {
    noteContent.style.padding = "0";
    noteContent.style.maxHeight = "0";
  }
  else
  {
    noteContent.style.padding = "10px 20px";
    noteContent.style.maxHeight = noteContent.scrollHeight + "px";
  }
  return !expanded;
}

function toggleNoteToggle(noteToggle)
{
  console.assert(noteToggle !== null, "noteToggle is null");
  console.assert(noteToggle.classList.contains("note-toggle"), "noteToggle is not a note-toggle");

  var expanded = isNoteExpanded(noteToggle);
  noteToggle.setAttribute("aria-expanded", !expanded);
  return !expanded;
}

function toggleNote(noteContainer)
{
  console.assert(noteContainer !== null, "noteContainer is null");
  console.assert(noteContainer.classList.contains("note-container"), "noteContainer is not a note-container");
  
  var noteToggle  = noteContainer.querySelector(".note-toggle");
  var noteContent = noteContainer.querySelector(".note-content");
  console.assert(noteToggle !== null, "noteToggle is null");
  console.assert(noteContent !== null, "noteContent is null");
  console.assert(noteToggle.nextElementSibling === noteContent, "noteToggle.nextElementSibling is not noteContent");

  toggleNoteToggle(noteToggle);
  toggleNoteContent(noteContent);
  return isNoteExpanded(noteContainer);
}

function getParentNoteContainer(element)
{
  console.assert(element !== null, "element is null");
  while (element !== null && !element.classList.contains("note-container"))
  {
    element = element.parentElement;
  }
  return element;
}

function getTogglableNotes()
{
  var toggles = document.querySelectorAll(".note-toggle");
  var notes = [];
  for (var i = 0; i < toggles.length; i++)
  {
    notes.push(getParentNoteContainer(toggles[i]));
  }
  return notes;
}

function registerNoteToggleClickHandlers()
{
  var toggles = document.querySelectorAll(".note-toggle");
  for (var i = 0; i < toggles.length; i++)
  {
    var toggle = toggles[i];
    toggle.addEventListener("click", function(event) {
      var noteContainer = getParentNoteContainer(event.target);
      toggleNote(noteContainer);
    });
  }
}

function initTogglableNote(noteContainer)
{
  var noteToggle  = noteContainer.querySelector(".note-toggle");
  var noteContent = noteContainer.querySelector(".note-content");
  console.assert(noteToggle !== null, "noteToggle is null");
  console.assert(noteContent !== null, "noteContent is null");
  console.assert(noteToggle.nextElementSibling === noteContent, "noteToggle.nextElementSibling is not noteContent");

  noteToggle.setAttribute("aria-expanded", "false");
  noteContent.style.maxHeight = "0";
  noteContent.style.overflow = "hidden";
  noteContent.style.transition = "max-height 0.3s ease-out";
  noteContent.style.transition += ", padding 0.3s ease-out";
  noteContent.style.display = "block";
  noteContent.classList.add("expandable");
}

function initNotes() {
  var notes = getTogglableNotes();
  for (var i = 0; i < notes.length; i++)
  {
    initTogglableNote(notes[i]);
  }
}

document.addEventListener("DOMContentLoaded", registerNoteToggleClickHandlers);
document.addEventListener("DOMContentLoaded", initNotes);
