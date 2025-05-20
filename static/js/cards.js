function getAllCards() {
  return document.querySelectorAll(".card");
}

function getCSSTransitionDuration({ element, ms = false }) {
  return (
    parseFloat(window.getComputedStyle(element).transitionDuration) *
    (ms ? 1000 : 1)
  );
}

document.addEventListener("DOMContentLoaded", function () {
  var cards = getAllCards();
  for (var i = 0; i < cards.length; i++) {
    var card = cards[i];
    card.style.zIndex = "1";
    card.style.overflow = "hidden";
    card.addEventListener("mouseenter", function (event) {
      console.assert(event.target.classList.contains("card"), "not a card");
      console.assert(event.target.style.zIndex !== "", "z-index is not set");
      console.assert(event.target.style.zIndex !== null, "z-index is not set");
      var oldZIndex = event.target.style.zIndex;
      var newZIndex = `${parseInt(oldZIndex) + 1}`;
      event.target.style.zIndex = newZIndex;
      event.target.style.overflow = "visible";
    });
    card.addEventListener("mouseleave", function (event) {
      console.assert(event.target.classList.contains("card"), "not a card");
      console.assert(event.target.style.zIndex !== "", "z-index is not set");
      console.assert(event.target.style.zIndex !== null, "z-index is not set");
      // sleep for the time of the transition
      // and then, reset z-index to normal
      var transitionDuration = getCSSTransitionDuration({
        element: event.target,
        ms: true,
      });
      setTimeout(function () {
        var oldZIndex = event.target.style.zIndex;
        var newZIndex = `${parseInt(oldZIndex) - 1}`;
        event.target.style.zIndex = newZIndex;
        event.target.style.overflow = "hidden";
      }, transitionDuration);
    });
  }
});
