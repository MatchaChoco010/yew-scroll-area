export function highlight() {
  hljs.highlightAll();
  hljs.highlightLinesAll([
    [
      {start: 5, end: 5, color: 'rgba(128, 255, 196, 0.3)'},
      {start: 7, end: 7, color: 'rgba(128, 255, 196, 0.3)'},
    ],
    // [{start: 3, end: 4, color: 'rgba(255, 255, 255, 0.2)'},],
  ]);
}
