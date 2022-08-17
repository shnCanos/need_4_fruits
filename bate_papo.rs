// * Written on fruit_plugin.rs
// Será que agora dá? Tenta aí quando acabares de almoçar

// nós temos um problmão no jogo
// é um problema bem grande
// a ordem de execução das coisas é totalmente aleatoria entao cada vez q vc joga o jogo, tem bugs diferentes // LOL
// eu percebi q isso sempre acontece lol Seria fixe se pudessemos definir uma ordem de execução mas não sei se é possível.
// Eles dizem no bevy book que prioritizam paralelismo

// lembra daquele problema do shaking? aquilo acontece as vezes, e as vezes não acontece
// hmmmmm. acho que é possível. eu tentei fazer, mas não consegui fazer funcionar do jeito q eu queria
// eu vou pôr o link no discord
// https://bevy-cheatbook.github.io/programming/system-order.html hmmm
// ah ye isso eu tentei fazer isso, mas não parecia fazer muita diferença.

// fun fact tho: aquele problema das frutas disaparecerem instantaneamente, é causado por essa aleatoriedade na ordem
// Mas eu é que programei isso de propósito, isso é uma feature
// se vc rodar o jogo várias vezes, ele as vezes não existe
// no I mean as frutas desaparecerem assim que spawnam. as vezes não acontece.
// ve o meu cursor