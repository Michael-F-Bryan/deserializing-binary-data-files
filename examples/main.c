#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

typedef uint16_t WORD;

typedef struct
{
    char name[2][20];
    char addr1[40];
    char addr2[40];
    char phone[16];
    uint16_t flags;
} spkr;

spkr generate();
spkr load(const char *filename);
void print_spkr(const spkr *speaker);
void save(const char *filename, const spkr *speaker);
void usage(const char *program);

int main(int argc, char **argv)
{
    if (argc == 3 && strcmp(argv[1], "generate") == 0)
    {
        spkr speaker = generate();
        save(argv[2], &speaker);
    }
    else if (argc == 3 && strcmp(argv[1], "load") == 0)
    {
        spkr speaker = load(argv[2]);
        print_spkr(&speaker);
    }
    else
    {
        usage(argv[0]);
    }

    return 0;
}

void usage(const char *program)
{
    fprintf(stderr, "Usage:\n");
    fprintf(stderr, "\t%s generate <output>\twrite some dumy data to a file\n", program);
    fprintf(stderr, "\t%s load <filename>\t\tprint the contents of a file\n", program);
}

// Generate a "spkr" populated with dummy data.
spkr generate()
{
    spkr speaker = {0};
    strncpy(speaker.name[0], "Joseph", sizeof(speaker.name[0]) - 1);
    strncpy(speaker.name[1], "Blogs", sizeof(speaker.name[1]) - 1);
    strncpy(speaker.addr1, "123 Fake Street", sizeof(speaker.addr1) - 1);
    strncpy(speaker.addr2, "New York", sizeof(speaker.addr2) - 1);
    strncpy(speaker.phone, "202-555-0117", sizeof(speaker.phone) - 1);
    speaker.flags = 0xAA0F;

    return speaker;
}

// Print a "spkr" to stdout.
void print_spkr(const spkr *speaker)
{
    printf("Name: %s %s\n", speaker->name[0], speaker->name[1]);

    printf("Address:\n");
    printf("\t%s\n", speaker->addr1);
    printf("\t%s\n", speaker->addr2);

    printf("Phone: %s\n", speaker->phone);
    printf("Flags: 0x%04X\n", speaker->flags);
}

// Read a "spkr" from a file.
spkr load(const char *filename)
{
    FILE *f = fopen(filename, "r");
    assert(f);

    spkr speaker = {0};
    fread(&speaker, sizeof(spkr), 1, f);
    fclose(f);

    return speaker;
}

// Save a "spkr" to a file.
void save(const char *filename, const spkr *speaker)
{
    FILE *f = fopen(filename, "w");

    fwrite(speaker, sizeof(spkr), 1, f);
    fclose(f);
}
