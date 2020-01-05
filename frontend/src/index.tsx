import React from 'react';
import ReactDom from 'react-dom';
import App from './App';

const target = document.createElement('div');
document.body.appendChild(target);

ReactDom.render((<App />), target);
