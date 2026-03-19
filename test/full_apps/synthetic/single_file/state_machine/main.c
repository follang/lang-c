#define MAX_STATES 4

typedef enum State {
    STATE_IDLE = 0,
    STATE_RUNNING = 1,
    STATE_ERROR = 2,
    STATE_DONE = 3
} State;

typedef struct Transition {
    int event;
    State next;
} Transition;

static const Transition TRANSITIONS[MAX_STATES][2] = {
    [STATE_IDLE] = {
        { 1, STATE_RUNNING },
        { 9, STATE_ERROR }
    },
    [STATE_RUNNING] = {
        { 2, STATE_DONE },
        { 9, STATE_ERROR }
    },
    [STATE_ERROR] = {
        { 0, STATE_IDLE },
        { 9, STATE_ERROR }
    },
    [STATE_DONE] = {
        { 0, STATE_IDLE },
        { 9, STATE_ERROR }
    }
};

static State apply_transition(State current, int event) {
    int i;
    for (i = 0; i < 2; ++i) {
        if (TRANSITIONS[current][i].event == event) {
            return TRANSITIONS[current][i].next;
        }
    }
    return STATE_ERROR;
}

int main(void) {
    int events[] = { 1, 2, 0 };
    State state = STATE_IDLE;
    int i;

    for (i = 0; i < 3; ++i) {
        state = apply_transition(state, events[i]);
        if (state == STATE_ERROR) {
            return 1;
        }
    }

    return state == STATE_IDLE ? 0 : 2;
}
