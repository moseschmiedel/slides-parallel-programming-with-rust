#include <chrono>
#include <cstdio>
#include <thread>
#include <vector>

int main() {
    std::vector<std::thread> threads;
    int y = 0;
    const int THREADS = 10000;

    for (int i = 0; i < THREADS; i++) {
        threads.push_back(std::thread([&y]() {
            std::this_thread::sleep_for(std::chrono::milliseconds(10));
            y += 1;
        }));
    }

    for (int i = 0; i < THREADS; i++) {
        threads[i].join();
    }

    printf("y = %d\n", y);
}
