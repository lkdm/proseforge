

const useFocusEditor = () => {
  const handleButtonClick = () => {
      // Find the nearest .milkdown element
      const milkdownElement = document.querySelector('.milkdown .editor') as HTMLElement | null;

      console.log(milkdownElement);
      if (milkdownElement) {
        milkdownElement.focus();
      }
    };

  const handleArticleClick = (event: React.MouseEvent) => {
      const clickedElement = event.target as HTMLElement;
      // If the click is on a .milkdown element or inside it, do nothing
        if (clickedElement.closest('.milkdown')) {
          return;
        }

      // Find all .milkdown elements
      const milkdownElements = document.querySelectorAll('.milkdown .editor');

      // Get the last .milkdown element
      const lastMilkdownElement = milkdownElements[milkdownElements.length - 1] as HTMLElement | null;

      if (lastMilkdownElement) {
        // Focus the last .milkdown element
        lastMilkdownElement.focus();

        // Place the cursor at the end of the content
        const range = document.createRange();
        const sel = window.getSelection();

        if (sel && lastMilkdownElement) {
          range.selectNodeContents(lastMilkdownElement);
          range.collapse(false); // Move to the end of the content
          sel.removeAllRanges();
          sel.addRange(range);
        }
      }
    };

  return { handleButtonClick, handleArticleClick };
}

export default useFocusEditor;
