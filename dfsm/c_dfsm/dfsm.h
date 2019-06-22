#define MAX_STATES 16
#define ERR_NALFSYMBOL -7
#define ERR_UNDSYMBOL -1

typedef struct Dfsm {
    int *states;
    char *alphabet;
    int startState;
    int *finalStates;
    int nStates;
    int nFinalStates;
    int currentState;
}Dfsm;

void initStates(Dfsm *);
void printStates(Dfsm dfsm);
void initAlphabet(Dfsm *, char *);
void printAlphabet(Dfsm);
Dfsm newDfsm(int);