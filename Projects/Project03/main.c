#include <stdio.h>
#include <stdint.h>
#include <string.h>

typedef struct RedBlackTree RedBlackTree;

extern RedBlackTree *tree_create();
extern int tree_insert(RedBlackTree *tree, uint64_t key, const char *val);
extern int tree_contains(RedBlackTree *tree, uint64_t key);
extern int tree_get(RedBlackTree *tree, uint64_t key, char *buf, size_t buflen);
extern int tree_remove(RedBlackTree *tree, uint64_t key);
extern void tree_free(RedBlackTree *tree);
extern void tree_print_structure(RedBlackTree *tree);

/// Exemplary usage of the Red-Black Tree in C language.
/// Output can be verified using: https://www.cs.usfca.edu/~galles/visualization/RedBlack.html
int main()
{
    printf("--- Red Black Tree (C Implementation) ---\n");

    RedBlackTree *tree = tree_create();
    if (!tree)
    {
        printf("Error: Allocation failed\n");
        return 1;
    }

    printf("\n[1] Initializing with 1, 2, 3:\n");
    tree_insert(tree, 1, "One");
    tree_insert(tree, 2, "Two");
    tree_insert(tree, 3, "Three");
    tree_print_structure(tree);
    printf("\n");

    printf("\n[2] Inserting 4 and 5:\n");

    tree_insert(tree, 4, "Four");
    tree_print_structure(tree);
    printf("\n");

    tree_insert(tree, 5, "Five");
    tree_print_structure(tree);
    printf("\n");

    printf("\n[3] Inserting 6, 7, 8, 9, 10:\n");

    struct
    {
        uint64_t k;
        const char *v;
    } values[] = {
        {6, "Six"},
        {7, "Seven"},
        {8, "Eight"},
        {9, "Nine"},
        {10, "Ten"}};
    int num_values = sizeof(values) / sizeof(values[0]);

    for (int i = 0; i < num_values; i++)
    {
        tree_insert(tree, values[i].k, values[i].v);
        tree_print_structure(tree);
        printf("\n");
    }

    printf("\n[4] Verification check:\n");
    uint64_t check_key = 7;
    char buffer[128];

    if (tree_get(tree, check_key, buffer, sizeof(buffer)) != -1)
    {
        printf("\tTree correctly contains key %llu: '%s'\n", check_key, buffer);
    }
    else
    {
        printf("\tError: Key %llu missing!\n", check_key);
    }

    printf("\n[5] Removing node (key 4):\n");
    tree_remove(tree, 4);
    tree_print_structure(tree);
    printf("\n");

    printf("\n[6] Removing node (key 2):\n");
    tree_remove(tree, 2);
    tree_print_structure(tree);
    printf("\n");

    printf("\n[7] Removing node (key 10):\n");
    tree_remove(tree, 10);
    tree_print_structure(tree);
    printf("\n");

    tree_free(tree);

    return 0;
}