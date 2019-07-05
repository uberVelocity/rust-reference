#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

#include "dfsm.h"

void initStates(Dfsm *dfsm) {
    for (int i = 0; i < dfsm->nStates; i++) {
        dfsm->states[i] = i + 1;
    }
}

void initFinalStates(Dfsm *dfsm) {
    dfsm->finalStates = malloc(1 * sizeof(int));
    assert(dfsm->finalStates != NULL);
    dfsm->finalStates[0] = 5;
    dfsm->nFinalStates = 1;
}

void printStates(Dfsm dfsm) {
    for (int i = 0; i < dfsm.nStates; i++) {
        printf("state %d: %d\n", i, dfsm.states[i]);
    }
}

void printAlphabet(Dfsm dfsm) {
    int i = 0;
    while(dfsm.alphabet[i] != '\0') {
        printf("%c", dfsm.alphabet[i]);
        i++;
    }
    printf("\n");
}

/**
 * Initializes the accepted alphabet of the machine.
 */
void initAlphabet(Dfsm *dfsm, char *alphabet) {
    int alphabetSize = strlen(alphabet) + 1;
    printf("alphabet size = %d\n", alphabetSize);
    dfsm->alphabet = (char*)malloc(alphabetSize * sizeof(char));
    assert(alphabet != NULL);
    for (int i = 0; i < alphabetSize; i++) {
        dfsm->alphabet[i] = alphabet[i];
    }
}

/**
 * Initializes a new Discrete-finite-state-machine of size `nStates`.
 */
Dfsm newDfsm(int nStates) {
    printf("--- INIT DFSM ---\n");
    Dfsm dfsm;
    if (!nStates) {
        nStates = MAX_STATES;
    }

    dfsm.currentState = 0;
    dfsm.nStates = nStates;
    dfsm.states = (int*)malloc(dfsm.nStates * sizeof(int));
    assert(dfsm.states != NULL);
    initStates(&dfsm);
    initFinalStates(&dfsm);
    return dfsm;
}

int validSymbol(char* input, char* alphabet) {
    int flag = 0;
    int alphabetSize = strlen(alphabet);
    int inputSize = strlen(input);
    for (int i = 0; i < inputSize; i++) {
        flag = 0;
        for (int j = 0; j < alphabetSize; j++) {
            if (input[i] == alphabet[j]) {
                flag = 1;
            }
        }
        if (flag == 0)  return 0;
    }
    return 1;
}

/**
 * Given a state and a symbol, the function returns a new state.
 */
int deltaTransition(int state, char symbol, char *alphabet) {
    switch(state) {
        case 0:
        switch(symbol) {
            case 'm':
            return 1;
            break;
            default:
            return ERR_UNDSYMBOL;
        }
        break;
        case 1:
        switch(symbol) {
            case 'i':
            return 2;
            break;
            default:
            return ERR_UNDSYMBOL;
        }
        break;
        case 2:
        switch(symbol) {
            case 'h':
            return 3;
            break;
            default:
            return ERR_UNDSYMBOL;
        }
        break;
        case 3:
        switch(symbol) {
            case 'a':
            return 4;
            break;
            default:
            return ERR_UNDSYMBOL;
        }
        break;
        case 4:
        switch(symbol) {
            case 'i':
            return 5;
            break;
            default:
            return ERR_UNDSYMBOL;
        }
        break;
        default:
        return ERR_UNDSYMBOL;
    }
}

int arrivedFinalState(Dfsm *dfsm) {
    for (int i = 0; i < dfsm->nFinalStates; i++) {
        if (dfsm->currentState == dfsm->finalStates[i]) {
            return 1;
        }
    }
    return 0;
}

void validInput() {
    printf("--- VALID INPUT ---\n");
}

void invalidInput() {
    printf("--- INVALID INPUT ---\n");
}

void abortMachine(int err) {
    switch(err) {
        case ERR_NALFSYMBOL:
        printf("--- Machine aborted ---\n");
        printf("--- INPUT SYMBOL NOT IN ALPHABET ---\n");
        perror("--- MACHINE ERROR THROWN ---\n");
        exit(EXIT_FAILURE);
        break;
        case ERR_UNDSYMBOL:
        printf("--- Machine aborted ---\n");
        printf("--- UNDEFINED TRANSITION ---\n");
        perror("--- MACHINE ERROR THROWN ---\n");
        exit(EXIT_FAILURE);
        break;
        default:
        perror("--- DEFAULT ERROR ---\n");
        perror("--- MACHINE ERROR THROWN ---");
        exit(EXIT_FAILURE);
    }
}

/**
 * Decides whether the machine accepts the given input.
 */
void parseInput(Dfsm *dfsm, int argc, char* argv[]) {
    printf("--- parsing input ---\n");
    int inputSize = strlen(argv[1]);
    for (int i = 0; i < inputSize; i++) {
        int newState = deltaTransition(dfsm->currentState, argv[1][i], dfsm->alphabet);
        if (newState == -1) {
            abortMachine(ERR_UNDSYMBOL);
        }
        dfsm->currentState = newState;
    }
    arrivedFinalState(dfsm) ? validInput() : invalidInput();
}

int main(int argc, char* argv[]) {
    if (!argv[1]) {
        perror("Input error: Empty argument given. Insert new non-empty input.\n");
        exit(EXIT_FAILURE);
    }

    Dfsm dfsm = newDfsm(MAX_STATES);

    initAlphabet(&dfsm, "abcdefghijklmnopqrstuvwxyz");
    if (!validSymbol(argv[1], dfsm.alphabet)) abortMachine(ERR_NALFSYMBOL);

    printAlphabet(dfsm);

    parseInput(&dfsm, argc, argv);

    free(dfsm.states);
    free(dfsm.alphabet);
    free(dfsm.finalStates);
}