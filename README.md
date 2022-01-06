# Tetris Game

# Development Steps

1)  First step to initialize the project: ```cargo init --lib tetris```.
2)  Create a directory ```tetris/www```.

Move in to the ```tetris/www``` directory.

3)  Initialize a node-project: ```npm init -y```.
4)  Install webpack ```npm install --save webpack webpack-cli copy-webpack-plugin```.
5)  Install development server: ```npm install --save-dev webpack-dev-server```.
6)  Create a ```.gitignore``` in ```www``` to avoid storing ```node_modules```.
7)  Create a sub-directory ```public```.
8)  Create ```index.html```, ```index.js``` and ```bootstrap.js``` in ```www```.
9)  Create and configure ```webpack.config.js```.
10) Add ```"dev": "webpack-dev-server"``` to the node configuration (```package.json```).

In order to use typescript (from ~/www):

1) ```npm install --save typescript ts-loader```.
2) Configure ```tsconfig.json```.