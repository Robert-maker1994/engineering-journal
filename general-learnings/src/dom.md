# Document Object Model (DOM)

The Document Object Model (DOM) is a programming interface for web documents. It represents the page so that programs can change the document structure, style, and content. The DOM represents the document as nodes and objects; that way, programming languages can interact with the page.

## Key Concepts

- **Nodes**: Every part of the document is a node. This includes elements, attributes, and text.
- **Tree Structure**: The DOM represents the document as a tree structure. The document's root is the `document` object, and all other nodes are children of this root.
- **Manipulation**: The DOM allows scripts to update the content, structure, and style of a document while it is being viewed.

## Common Operations

- **Selecting Elements**: Methods like `getElementById`, `getElementsByClassName`, and `querySelector` are used to select elements.
- **Modifying Content**: Properties like `innerHTML`, `textContent`, and methods like `appendChild`, `removeChild` are used to modify the document.
- **Event Handling**: The DOM allows the registration of event handlers to elements to respond to user actions like clicks, key presses, etc.

## Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>DOM Example</title>
</head>
<body>
    <div id="content">Hello, World!</div>
    <script>
        document.getElementById('content').textContent = 'Hello, DOM!';
    </script>
</body>
</html>
```

In this example, the JavaScript code changes the text content of the `div` element with the id `content` from "Hello, World!" to "Hello, DOM!".
