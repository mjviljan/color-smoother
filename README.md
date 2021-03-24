# Color Smoother

This is a small project done to practice using Rust 🦀 and Wasm 🕸 together.

## The idea

When the app/game is launched, a rectangular area is filled with square (or rectangular?) cells of random shades of a particular color. The user can then advance the game universe one step – or tick – at a time, and each of the cells evolves to a possibly new state.

The cells evolve by trying to blend in into their surroundings. They do this by adjusting their shade to better match that of their neighbours', calculating the average shade of all of their neighbours. Cardinal neighbours have a bigger weight in the calculation than diagonal neighbours. In one tick each cell can only change its shade by one step in the direction of their target shade. The target shade is calculated again on each tick and can vary over time as the shades of the cell's neighbours also evolve using the same logic.

My hypothesis is that the cells of the universe should blend into their neighbours so that the universe will become significantly smoother over time, possibly reaching an equilibrium of sorts where no cell changes its shade anymore. On the other hand it's also possible that neighbouring cells switch back and forth between two shades because of their neighbour doing the same (in the other direction), and they keep doing this forever.

The whole idea of doing a graphical presentation of this stems from my curiosity to see what actually happens. I could find out by doing some calculations manually and without any visual output but that wouldn't be nearly as fun.  