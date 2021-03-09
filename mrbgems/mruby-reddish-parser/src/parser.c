/* A Bison parser, made by GNU Bison 3.7.  */

/* Bison implementation for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2020 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */

/* Identify Bison output.  */
#define YYBISON 1

/* Bison version.  */
#define YYBISON_VERSION "3.7"

/* Skeleton name.  */
#define YYSKELETON_NAME "yacc.c"

/* Pure parsers.  */
#define YYPURE 1

/* Push parsers.  */
#define YYPUSH 0

/* Pull parsers.  */
#define YYPULL 1




/* First part of user prologue.  */
#line 1 "mrbgems/mruby-reddish-parser/core/parser.y"

#include <string.h>
#include <stdio.h>
#include "mruby.h"
#include "mruby/string.h"
#include "mruby/array.h"
#include "mruby/variable.h"

typedef struct parser_state {
    mrb_state* state;
    mrb_value  result;
    mrb_value  action;
    mrb_value  lexer;
} parser_state;

#define ACTION(p, n, c, ...)   mrb_funcall(p->state, p->action, n, c, __VA_ARGS__)
#define APPEND_REDIRECT(p, s, r)  ACTION(p, "on_append_redirect", 2, s, r)
#define ASYNC(p, c)               ACTION(p, "on_async", 1, c)
#define COMMAND(p, e)             ACTION(p, "on_command", 1, e)
#define CONNECTOR(p, t, a, b)     ACTION(p, "on_connector", 3, mrb_symbol_value(mrb_intern_cstr(p->state, t)), a, b)
#define FOR_STMT(p, n, k, c)      ACTION(p, "on_for_stmt", 3, n, k, c)
#define IF_STMT(p, s, c, ...)     ACTION(p, "on_if_stmt", (c+2), s, MRB_FALSE, __VA_ARGS__)
#define PIPELINE(p, a, b, r)      ACTION(p, "on_pipeline", 3, a, b, r)
#define REDIRECT(p, t, c, ...)    ACTION(p, "on_redirect", (c+1), mrb_symbol_value(mrb_intern_cstr(p->state, t)), __VA_ARGS__)
#define UNLESS_STMT(p, s, c, ...) ACTION(p, "on_if_stmt", (c+2), s, MRB_TRUE, __VA_ARGS__)
#define UNTIL_STMT(p, s, c)       ACTION(p, "on_while_stmt", 3, s, MRB_TRUE, c)
#define WHILE_STMT(p, s, c)       ACTION(p, "on_while_stmt", 3, s, MRB_FALSE, c)
#define WORD(p, w)                ACTION(p, "on_word", 1, w)

#define FIXNUM(i) mrb_fixnum_value(i)
#define MRB_TRUE  mrb_true_value()
#define MRB_FALSE mrb_false_value()
#define NIL       mrb_nil_value()

#define YYDEBUG 1

static int yylex(void* , parser_state*);
static void yyerror(parser_state*, const char*);
static int yyparse(parser_state*);

#line 112 "mrbgems/mruby-reddish-parser/src/parser.c"

# ifndef YY_CAST
#  ifdef __cplusplus
#   define YY_CAST(Type, Val) static_cast<Type> (Val)
#   define YY_REINTERPRET_CAST(Type, Val) reinterpret_cast<Type> (Val)
#  else
#   define YY_CAST(Type, Val) ((Type) (Val))
#   define YY_REINTERPRET_CAST(Type, Val) ((Type) (Val))
#  endif
# endif
# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif


/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int yydebug;
#endif

/* Token kinds.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    YYEMPTY = -2,
    YYEOF = 0,                     /* "end of file"  */
    YYerror = 256,                 /* error  */
    YYUNDEF = 257,                 /* "invalid token"  */
    WORD = 258,                    /* WORD  */
    NUMBER = 259,                  /* NUMBER  */
    MINUS = 260,                   /* MINUS  */
    NUMBER_MINUS = 261,            /* NUMBER_MINUS  */
    IF = 262,                      /* IF  */
    THEN = 263,                    /* THEN  */
    ELSE = 264,                    /* ELSE  */
    ELIF = 265,                    /* ELIF  */
    ELSIF = 266,                   /* ELSIF  */
    FI = 267,                      /* FI  */
    END = 268,                     /* END  */
    UNLESS = 269,                  /* UNLESS  */
    WHILE = 270,                   /* WHILE  */
    DO = 271,                      /* DO  */
    DONE = 272,                    /* DONE  */
    UNTIL = 273,                   /* UNTIL  */
    FOR = 274,                     /* FOR  */
    IN = 275,                      /* IN  */
    AND_AND = 276,                 /* AND_AND  */
    OR_OR = 277,                   /* OR_OR  */
    OR_AND = 278,                  /* OR_AND  */
    GT = 279,                      /* GT  */
    GT_GT = 280,                   /* GT_GT  */
    AND_GT = 281,                  /* AND_GT  */
    AND_GT_GT = 282,               /* AND_GT_GT  */
    GT_AND = 283,                  /* GT_AND  */
    GT_GT_AND = 284,               /* GT_GT_AND  */
    LT = 285,                      /* LT  */
    LT_AND = 286,                  /* LT_AND  */
    LT_GT = 287                    /* LT_GT  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
typedef mrb_value YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif



int yyparse (parser_state* p);


/* Symbol kind.  */
enum yysymbol_kind_t
{
  YYSYMBOL_YYEMPTY = -2,
  YYSYMBOL_YYEOF = 0,                      /* "end of file"  */
  YYSYMBOL_YYerror = 1,                    /* error  */
  YYSYMBOL_YYUNDEF = 2,                    /* "invalid token"  */
  YYSYMBOL_WORD = 3,                       /* WORD  */
  YYSYMBOL_NUMBER = 4,                     /* NUMBER  */
  YYSYMBOL_MINUS = 5,                      /* MINUS  */
  YYSYMBOL_NUMBER_MINUS = 6,               /* NUMBER_MINUS  */
  YYSYMBOL_IF = 7,                         /* IF  */
  YYSYMBOL_THEN = 8,                       /* THEN  */
  YYSYMBOL_ELSE = 9,                       /* ELSE  */
  YYSYMBOL_ELIF = 10,                      /* ELIF  */
  YYSYMBOL_ELSIF = 11,                     /* ELSIF  */
  YYSYMBOL_FI = 12,                        /* FI  */
  YYSYMBOL_END = 13,                       /* END  */
  YYSYMBOL_UNLESS = 14,                    /* UNLESS  */
  YYSYMBOL_WHILE = 15,                     /* WHILE  */
  YYSYMBOL_DO = 16,                        /* DO  */
  YYSYMBOL_DONE = 17,                      /* DONE  */
  YYSYMBOL_UNTIL = 18,                     /* UNTIL  */
  YYSYMBOL_FOR = 19,                       /* FOR  */
  YYSYMBOL_IN = 20,                        /* IN  */
  YYSYMBOL_AND_AND = 21,                   /* AND_AND  */
  YYSYMBOL_OR_OR = 22,                     /* OR_OR  */
  YYSYMBOL_OR_AND = 23,                    /* OR_AND  */
  YYSYMBOL_GT = 24,                        /* GT  */
  YYSYMBOL_GT_GT = 25,                     /* GT_GT  */
  YYSYMBOL_AND_GT = 26,                    /* AND_GT  */
  YYSYMBOL_AND_GT_GT = 27,                 /* AND_GT_GT  */
  YYSYMBOL_GT_AND = 28,                    /* GT_AND  */
  YYSYMBOL_GT_GT_AND = 29,                 /* GT_GT_AND  */
  YYSYMBOL_LT = 30,                        /* LT  */
  YYSYMBOL_LT_AND = 31,                    /* LT_AND  */
  YYSYMBOL_LT_GT = 32,                     /* LT_GT  */
  YYSYMBOL_33_n_ = 33,                     /* '\n'  */
  YYSYMBOL_34_ = 34,                       /* '&'  */
  YYSYMBOL_35_ = 35,                       /* ';'  */
  YYSYMBOL_36_ = 36,                       /* '|'  */
  YYSYMBOL_37_ = 37,                       /* '{'  */
  YYSYMBOL_38_ = 38,                       /* '}'  */
  YYSYMBOL_YYACCEPT = 39,                  /* $accept  */
  YYSYMBOL_inputunit = 40,                 /* inputunit  */
  YYSYMBOL_terminator = 41,                /* terminator  */
  YYSYMBOL_simple_list = 42,               /* simple_list  */
  YYSYMBOL_connector = 43,                 /* connector  */
  YYSYMBOL_pipeline = 44,                  /* pipeline  */
  YYSYMBOL_compound_list = 45,             /* compound_list  */
  YYSYMBOL_list = 46,                      /* list  */
  YYSYMBOL_list0 = 47,                     /* list0  */
  YYSYMBOL_list1 = 48,                     /* list1  */
  YYSYMBOL_newline_list = 49,              /* newline_list  */
  YYSYMBOL_command = 50,                   /* command  */
  YYSYMBOL_shell_command = 51,             /* shell_command  */
  YYSYMBOL_if_statement = 52,              /* if_statement  */
  YYSYMBOL_elif_clause = 53,               /* elif_clause  */
  YYSYMBOL_elsif_clause = 54,              /* elsif_clause  */
  YYSYMBOL_unless_statement = 55,          /* unless_statement  */
  YYSYMBOL_while_statement = 56,           /* while_statement  */
  YYSYMBOL_until_statement = 57,           /* until_statement  */
  YYSYMBOL_for_statement = 58,             /* for_statement  */
  YYSYMBOL_simple_command = 59,            /* simple_command  */
  YYSYMBOL_simple_command_element = 60,    /* simple_command_element  */
  YYSYMBOL_redirect_list = 61,             /* redirect_list  */
  YYSYMBOL_word_list = 62,                 /* word_list  */
  YYSYMBOL_list_terminater = 63,           /* list_terminater  */
  YYSYMBOL_redirect = 64                   /* redirect  */
};
typedef enum yysymbol_kind_t yysymbol_kind_t;




#ifdef short
# undef short
#endif

/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */

#ifndef __PTRDIFF_MAX__
# include <limits.h> /* INFRINGES ON USER NAME SPACE */
# if defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stdint.h> /* INFRINGES ON USER NAME SPACE */
#  define YY_STDINT_H
# endif
#endif

/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */

#ifdef __INT_LEAST8_MAX__
typedef __INT_LEAST8_TYPE__ yytype_int8;
#elif defined YY_STDINT_H
typedef int_least8_t yytype_int8;
#else
typedef signed char yytype_int8;
#endif

#ifdef __INT_LEAST16_MAX__
typedef __INT_LEAST16_TYPE__ yytype_int16;
#elif defined YY_STDINT_H
typedef int_least16_t yytype_int16;
#else
typedef short yytype_int16;
#endif

#if defined __UINT_LEAST8_MAX__ && __UINT_LEAST8_MAX__ <= __INT_MAX__
typedef __UINT_LEAST8_TYPE__ yytype_uint8;
#elif (!defined __UINT_LEAST8_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST8_MAX <= INT_MAX)
typedef uint_least8_t yytype_uint8;
#elif !defined __UINT_LEAST8_MAX__ && UCHAR_MAX <= INT_MAX
typedef unsigned char yytype_uint8;
#else
typedef short yytype_uint8;
#endif

#if defined __UINT_LEAST16_MAX__ && __UINT_LEAST16_MAX__ <= __INT_MAX__
typedef __UINT_LEAST16_TYPE__ yytype_uint16;
#elif (!defined __UINT_LEAST16_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST16_MAX <= INT_MAX)
typedef uint_least16_t yytype_uint16;
#elif !defined __UINT_LEAST16_MAX__ && USHRT_MAX <= INT_MAX
typedef unsigned short yytype_uint16;
#else
typedef int yytype_uint16;
#endif

#ifndef YYPTRDIFF_T
# if defined __PTRDIFF_TYPE__ && defined __PTRDIFF_MAX__
#  define YYPTRDIFF_T __PTRDIFF_TYPE__
#  define YYPTRDIFF_MAXIMUM __PTRDIFF_MAX__
# elif defined PTRDIFF_MAX
#  ifndef ptrdiff_t
#   include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  endif
#  define YYPTRDIFF_T ptrdiff_t
#  define YYPTRDIFF_MAXIMUM PTRDIFF_MAX
# else
#  define YYPTRDIFF_T long
#  define YYPTRDIFF_MAXIMUM LONG_MAX
# endif
#endif

#ifndef YYSIZE_T
# ifdef __SIZE_TYPE__
#  define YYSIZE_T __SIZE_TYPE__
# elif defined size_t
#  define YYSIZE_T size_t
# elif defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  define YYSIZE_T size_t
# else
#  define YYSIZE_T unsigned
# endif
#endif

#define YYSIZE_MAXIMUM                                  \
  YY_CAST (YYPTRDIFF_T,                                 \
           (YYPTRDIFF_MAXIMUM < YY_CAST (YYSIZE_T, -1)  \
            ? YYPTRDIFF_MAXIMUM                         \
            : YY_CAST (YYSIZE_T, -1)))

#define YYSIZEOF(X) YY_CAST (YYPTRDIFF_T, sizeof (X))


/* Stored state numbers (used for stacks). */
typedef yytype_uint8 yy_state_t;

/* State numbers in computations.  */
typedef int yy_state_fast_t;

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(Msgid) dgettext ("bison-runtime", Msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(Msgid) Msgid
# endif
#endif


#ifndef YY_ATTRIBUTE_PURE
# if defined __GNUC__ && 2 < __GNUC__ + (96 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_PURE __attribute__ ((__pure__))
# else
#  define YY_ATTRIBUTE_PURE
# endif
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# if defined __GNUC__ && 2 < __GNUC__ + (7 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_UNUSED __attribute__ ((__unused__))
# else
#  define YY_ATTRIBUTE_UNUSED
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YYUSE(E) ((void) (E))
#else
# define YYUSE(E) /* empty */
#endif

#if defined __GNUC__ && ! defined __ICC && 407 <= __GNUC__ * 100 + __GNUC_MINOR__
/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                            \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")              \
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# define YY_IGNORE_MAYBE_UNINITIALIZED_END      \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

#if defined __cplusplus && defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_USELESS_CAST_BEGIN                          \
    _Pragma ("GCC diagnostic push")                            \
    _Pragma ("GCC diagnostic ignored \"-Wuseless-cast\"")
# define YY_IGNORE_USELESS_CAST_END            \
    _Pragma ("GCC diagnostic pop")
#endif
#ifndef YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_END
#endif


#define YY_ASSERT(E) ((void) (0 && (E)))

#if !defined yyoverflow

/* The parser invokes alloca or malloc; define the necessary symbols.  */

# ifdef YYSTACK_USE_ALLOCA
#  if YYSTACK_USE_ALLOCA
#   ifdef __GNUC__
#    define YYSTACK_ALLOC __builtin_alloca
#   elif defined __BUILTIN_VA_ARG_INCR
#    include <alloca.h> /* INFRINGES ON USER NAME SPACE */
#   elif defined _AIX
#    define YYSTACK_ALLOC __alloca
#   elif defined _MSC_VER
#    include <malloc.h> /* INFRINGES ON USER NAME SPACE */
#    define alloca _alloca
#   else
#    define YYSTACK_ALLOC alloca
#    if ! defined _ALLOCA_H && ! defined EXIT_SUCCESS
#     include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
      /* Use EXIT_SUCCESS as a witness for stdlib.h.  */
#     ifndef EXIT_SUCCESS
#      define EXIT_SUCCESS 0
#     endif
#    endif
#   endif
#  endif
# endif

# ifdef YYSTACK_ALLOC
   /* Pacify GCC's 'empty if-body' warning.  */
#  define YYSTACK_FREE(Ptr) do { /* empty */; } while (0)
#  ifndef YYSTACK_ALLOC_MAXIMUM
    /* The OS might guarantee only one guard page at the bottom of the stack,
       and a page size can be as small as 4096 bytes.  So we cannot safely
       invoke alloca (N) if N exceeds 4096.  Use a slightly smaller number
       to allow for a few compiler-allocated temporary stack slots.  */
#   define YYSTACK_ALLOC_MAXIMUM 4032 /* reasonable circa 2006 */
#  endif
# else
#  define YYSTACK_ALLOC YYMALLOC
#  define YYSTACK_FREE YYFREE
#  ifndef YYSTACK_ALLOC_MAXIMUM
#   define YYSTACK_ALLOC_MAXIMUM YYSIZE_MAXIMUM
#  endif
#  if (defined __cplusplus && ! defined EXIT_SUCCESS \
       && ! ((defined YYMALLOC || defined malloc) \
             && (defined YYFREE || defined free)))
#   include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#   ifndef EXIT_SUCCESS
#    define EXIT_SUCCESS 0
#   endif
#  endif
#  ifndef YYMALLOC
#   define YYMALLOC malloc
#   if ! defined malloc && ! defined EXIT_SUCCESS
void *malloc (YYSIZE_T); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
#  ifndef YYFREE
#   define YYFREE free
#   if ! defined free && ! defined EXIT_SUCCESS
void free (void *); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
# endif
#endif /* !defined yyoverflow */

#if (! defined yyoverflow \
     && (! defined __cplusplus \
         || (defined YYSTYPE_IS_TRIVIAL && YYSTYPE_IS_TRIVIAL)))

/* A type that is properly aligned for any stack member.  */
union yyalloc
{
  yy_state_t yyss_alloc;
  YYSTYPE yyvs_alloc;
};

/* The size of the maximum gap between one aligned stack and the next.  */
# define YYSTACK_GAP_MAXIMUM (YYSIZEOF (union yyalloc) - 1)

/* The size of an array large to enough to hold all stacks, each with
   N elements.  */
# define YYSTACK_BYTES(N) \
     ((N) * (YYSIZEOF (yy_state_t) + YYSIZEOF (YYSTYPE)) \
      + YYSTACK_GAP_MAXIMUM)

# define YYCOPY_NEEDED 1

/* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
# define YYSTACK_RELOCATE(Stack_alloc, Stack)                           \
    do                                                                  \
      {                                                                 \
        YYPTRDIFF_T yynewbytes;                                         \
        YYCOPY (&yyptr->Stack_alloc, Stack, yysize);                    \
        Stack = &yyptr->Stack_alloc;                                    \
        yynewbytes = yystacksize * YYSIZEOF (*Stack) + YYSTACK_GAP_MAXIMUM; \
        yyptr += yynewbytes / YYSIZEOF (*yyptr);                        \
      }                                                                 \
    while (0)

#endif

#if defined YYCOPY_NEEDED && YYCOPY_NEEDED
/* Copy COUNT objects from SRC to DST.  The source and destination do
   not overlap.  */
# ifndef YYCOPY
#  if defined __GNUC__ && 1 < __GNUC__
#   define YYCOPY(Dst, Src, Count) \
      __builtin_memcpy (Dst, Src, YY_CAST (YYSIZE_T, (Count)) * sizeof (*(Src)))
#  else
#   define YYCOPY(Dst, Src, Count)              \
      do                                        \
        {                                       \
          YYPTRDIFF_T yyi;                      \
          for (yyi = 0; yyi < (Count); yyi++)   \
            (Dst)[yyi] = (Src)[yyi];            \
        }                                       \
      while (0)
#  endif
# endif
#endif /* !YYCOPY_NEEDED */

/* YYFINAL -- State number of the termination state.  */
#define YYFINAL  57
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   253

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  39
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  26
/* YYNRULES -- Number of rules.  */
#define YYNRULES  113
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  202

/* YYMAXUTOK -- Last valid token kind.  */
#define YYMAXUTOK   287


/* YYTRANSLATE(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, with out-of-bounds checking.  */
#define YYTRANSLATE(YYX)                                \
  (0 <= (YYX) && (YYX) <= YYMAXUTOK                     \
   ? YY_CAST (yysymbol_kind_t, yytranslate[YYX])        \
   : YYSYMBOL_YYUNDEF)

/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static const yytype_int8 yytranslate[] =
{
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
      33,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,    34,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,    35,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,    37,    36,    38,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    27,    28,    29,    30,    31,    32
};

#if YYDEBUG
  /* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
static const yytype_uint8 yyrline[] =
{
       0,    57,    57,    58,    60,    60,    63,    64,    65,    68,
      69,    70,    71,    72,    75,    76,    77,    80,    81,    84,
      87,    88,    89,    92,    93,    94,    95,    96,    97,   100,
     101,   104,   105,   106,   109,   110,   111,   112,   113,   116,
     117,   118,   119,   120,   121,   122,   123,   124,   125,   126,
     129,   130,   131,   132,   135,   136,   137,   138,   139,   140,
     143,   144,   145,   146,   149,   150,   151,   154,   155,   158,
     159,   160,   161,   162,   163,   164,   165,   166,   167,   170,
     171,   174,   175,   178,   179,   182,   183,   185,   185,   185,
     188,   189,   190,   191,   192,   193,   194,   195,   196,   197,
     198,   199,   200,   201,   202,   203,   204,   205,   206,   207,
     208,   209,   210,   211
};
#endif

/** Accessing symbol of state STATE.  */
#define YY_ACCESSING_SYMBOL(State) YY_CAST (yysymbol_kind_t, yystos[State])

#if YYDEBUG || 0
/* The user-facing name of the symbol whose (internal) number is
   YYSYMBOL.  No bounds checking.  */
static const char *yysymbol_name (yysymbol_kind_t yysymbol) YY_ATTRIBUTE_UNUSED;

/* YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
   First, the terminals, then, starting at YYNTOKENS, nonterminals.  */
static const char *const yytname[] =
{
  "\"end of file\"", "error", "\"invalid token\"", "WORD", "NUMBER",
  "MINUS", "NUMBER_MINUS", "IF", "THEN", "ELSE", "ELIF", "ELSIF", "FI",
  "END", "UNLESS", "WHILE", "DO", "DONE", "UNTIL", "FOR", "IN", "AND_AND",
  "OR_OR", "OR_AND", "GT", "GT_GT", "AND_GT", "AND_GT_GT", "GT_AND",
  "GT_GT_AND", "LT", "LT_AND", "LT_GT", "'\\n'", "'&'", "';'", "'|'",
  "'{'", "'}'", "$accept", "inputunit", "terminator", "simple_list",
  "connector", "pipeline", "compound_list", "list", "list0", "list1",
  "newline_list", "command", "shell_command", "if_statement",
  "elif_clause", "elsif_clause", "unless_statement", "while_statement",
  "until_statement", "for_statement", "simple_command",
  "simple_command_element", "redirect_list", "word_list",
  "list_terminater", "redirect", YY_NULLPTR
};

static const char *
yysymbol_name (yysymbol_kind_t yysymbol)
{
  return yytname[yysymbol];
}
#endif

#ifdef YYPRINT
/* YYTOKNUM[NUM] -- (External) token number corresponding to the
   (internal) symbol number NUM (which must be that of a token).  */
static const yytype_int16 yytoknum[] =
{
       0,   256,   257,   258,   259,   260,   261,   262,   263,   264,
     265,   266,   267,   268,   269,   270,   271,   272,   273,   274,
     275,   276,   277,   278,   279,   280,   281,   282,   283,   284,
     285,   286,   287,    10,    38,    59,   124,   123,   125
};
#endif

#define YYPACT_NINF (-153)

#define yypact_value_is_default(Yyn) \
  ((Yyn) == YYPACT_NINF)

#define YYTABLE_NINF (-1)

#define yytable_value_is_error(Yyn) \
  0

  /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static const yytype_int16 yypact[] =
{
     139,  -153,  -153,   211,  -153,  -153,  -153,  -153,    20,    22,
      32,    34,    68,    96,    82,    84,    45,   149,    11,    10,
      94,    -3,  -153,    80,  -153,  -153,  -153,  -153,  -153,    66,
    -153,  -153,   153,   169,   117,   184,   174,   203,     8,  -153,
     201,    55,    31,   167,    -6,  -153,  -153,  -153,  -153,  -153,
     132,  -153,  -153,  -153,   204,  -153,  -153,  -153,  -153,  -153,
    -153,  -153,  -153,    27,    27,    27,    27,    80,  -153,  -153,
    -153,  -153,   232,  -153,  -153,   233,  -153,  -153,  -153,  -153,
    -153,  -153,     0,  -153,    -3,  -153,    92,  -153,  -153,  -153,
    -153,  -153,  -153,  -153,   237,  -153,    -9,  -153,  -153,   201,
     201,    -3,    -3,  -153,  -153,  -153,  -153,  -153,   123,   231,
     202,  -153,  -153,  -153,    27,  -153,  -153,     9,   234,    48,
     235,  -153,     5,    -1,  -153,  -153,    -3,    -3,  -153,  -153,
    -153,  -153,   200,   205,  -153,  -153,  -153,   201,   201,    -3,
     181,   201,   201,  -153,  -153,  -153,  -153,  -153,  -153,  -153,
    -153,  -153,  -153,  -153,  -153,  -153,    59,   207,   209,   238,
    -153,  -153,  -153,  -153,   151,  -153,    -3,    -3,    -3,    -3,
     236,   239,   170,   134,   212,  -153,  -153,  -153,  -153,  -153,
    -153,  -153,  -153,  -153,  -153,  -153,  -153,  -153,  -153,  -153,
    -153,   182,  -153,   142,   213,  -153,  -153,  -153,  -153,  -153,
    -153,  -153
};

  /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static const yytype_int8 yydefact[] =
{
       0,     3,    81,     0,    29,    29,    29,    29,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       6,    13,    16,    32,    34,    35,    36,    37,    38,    31,
      79,    82,     0,     0,     0,     0,     0,     0,     0,    17,
       0,     0,     0,     0,    29,    98,   108,   106,   110,   107,
     101,   100,   111,    90,    93,    92,   112,     1,     5,     4,
       2,    29,    29,     7,     8,     0,     0,    33,    83,    80,
      99,   109,   104,   103,    91,    96,    95,   113,    29,    29,
      29,    39,     0,    30,    28,    19,    18,    29,    29,    60,
      64,    29,    67,    29,     0,    29,     0,   102,    94,     0,
       0,    11,    12,    15,    14,    84,   105,    97,     0,     0,
      54,    41,    29,    29,    29,    29,    29,     0,     0,     0,
       0,    85,     0,     0,    29,    29,     9,    10,    29,    29,
      42,    43,     0,     0,    40,    29,    29,     0,     0,    27,
      20,    21,    22,    29,    62,    61,    66,    65,    68,    89,
      86,    87,    88,    29,    29,    29,     0,     0,     0,     0,
      46,    47,    48,    49,    56,    55,    23,    24,    25,    26,
       0,     0,     0,     0,     0,    70,    69,    71,    44,    45,
      29,    29,    58,    59,    63,    75,    29,    29,    73,    72,
      74,    50,    57,     0,     0,    29,    52,    53,    77,    76,
      78,    51
};

  /* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -153,  -153,  -153,  -153,  -153,     3,    -5,  -153,  -153,  -153,
     -35,   158,  -153,  -153,  -152,  -102,  -153,  -153,  -153,  -153,
    -153,   224,  -153,  -153,  -153,   -19
};

  /* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int16 yydefgoto[] =
{
      -1,    18,    60,    19,    20,    84,    38,    39,    85,    86,
      40,    22,    23,    24,   132,    82,    25,    26,    27,    28,
      29,    30,    67,   122,   153,    31
};

  /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static const yytype_uint8 yytable[] =
{
      41,    42,    43,    21,    68,   149,   133,   124,   150,    96,
      58,    57,   182,   111,    94,   154,    78,    79,   143,    80,
      65,    81,   144,    44,    83,    45,    99,   100,   125,    95,
       2,     3,    83,    66,     4,    46,   155,    47,   151,   196,
     152,     5,     6,    59,    90,     7,     8,    91,   105,    54,
      55,     9,    10,    11,    12,    13,    14,    15,    16,    17,
     123,   146,   183,    87,    88,   147,   101,   102,    89,     2,
       3,    48,   175,   108,   109,   110,   176,   137,   138,   140,
     141,   142,   117,   118,     3,    52,   119,    53,   120,   197,
       9,    10,    11,    12,    13,    14,    15,    16,    17,    49,
      50,    51,   126,   127,     9,    10,    11,    12,    13,    14,
      15,    16,    17,   112,   113,    61,    62,   139,   172,   156,
     157,    72,    73,   158,   159,   114,   115,   116,    63,    64,
     164,   165,   128,   129,    80,   130,   131,    97,   170,     1,
     166,   167,     2,     3,   168,   169,     4,   188,   171,   173,
     174,   189,    56,     5,     6,   198,    70,     7,     8,   199,
     181,   129,    80,     9,    10,    11,    12,    13,    14,    15,
      16,    17,    71,     2,     3,   191,   192,     4,    75,    76,
      92,   193,   194,    93,     5,     6,   186,    74,     7,     8,
     201,   195,   129,    80,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    83,     2,     3,    77,   187,     4,    98,
     135,   136,   160,   161,    83,     5,     6,   162,   163,     7,
       8,   178,   179,   103,   104,     9,    10,    11,    12,    13,
      14,    15,    16,    17,    83,    32,    33,   106,   107,    34,
     121,    35,    36,    37,   134,   177,   180,   145,   148,   184,
     190,   200,   185,    69
};

static const yytype_uint8 yycheck[] =
{
       5,     6,     7,     0,    23,     0,   108,    16,     3,    44,
       0,     0,   164,    13,    20,    16,     8,     9,     9,    11,
      23,    13,    13,     3,    33,     3,    61,    62,    37,    35,
       3,     4,    33,    36,     7,     3,    37,     3,    33,   191,
      35,    14,    15,    33,    13,    18,    19,    16,    67,     4,
       5,    24,    25,    26,    27,    28,    29,    30,    31,    32,
      95,    13,   164,     8,     9,    17,    63,    64,    13,     3,
       4,     3,    13,    78,    79,    80,    17,   112,   113,   114,
     115,   116,    87,    88,     4,     3,    91,     3,    93,   191,
      24,    25,    26,    27,    28,    29,    30,    31,    32,     3,
       4,     5,    99,   100,    24,    25,    26,    27,    28,    29,
      30,    31,    32,    21,    22,    21,    22,   114,   153,   124,
     125,     4,     5,   128,   129,    33,    34,    35,    34,    35,
     135,   136,     9,    10,    11,    12,    13,     5,   143,     0,
     137,   138,     3,     4,   141,   142,     7,    13,   153,   154,
     155,    17,     3,    14,    15,    13,     3,    18,    19,    17,
       9,    10,    11,    24,    25,    26,    27,    28,    29,    30,
      31,    32,     3,     3,     4,   180,   181,     7,     4,     5,
      13,   186,   187,    16,    14,    15,    16,     3,    18,    19,
     195,     9,    10,    11,    24,    25,    26,    27,    28,    29,
      30,    31,    32,    33,     3,     4,     3,    37,     7,     5,
       8,     9,    12,    13,    33,    14,    15,    12,    13,    18,
      19,    12,    13,    65,    66,    24,    25,    26,    27,    28,
      29,    30,    31,    32,    33,    24,    25,     5,     5,    28,
       3,    30,    31,    32,    13,    38,     8,    13,    13,    13,
      38,    38,    13,    29
};

  /* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static const yytype_int8 yystos[] =
{
       0,     0,     3,     4,     7,    14,    15,    18,    19,    24,
      25,    26,    27,    28,    29,    30,    31,    32,    40,    42,
      43,    44,    50,    51,    52,    55,    56,    57,    58,    59,
      60,    64,    24,    25,    28,    30,    31,    32,    45,    46,
      49,    45,    45,    45,     3,     3,     3,     3,     3,     3,
       4,     5,     3,     3,     4,     5,     3,     0,     0,    33,
      41,    21,    22,    34,    35,    23,    36,    61,    64,    60,
       3,     3,     4,     5,     3,     4,     5,     3,     8,     9,
      11,    13,    54,    33,    44,    47,    48,     8,     9,    13,
      13,    16,    13,    16,    20,    35,    49,     5,     5,    49,
      49,    44,    44,    50,    50,    64,     5,     5,    45,    45,
      45,    13,    21,    22,    33,    34,    35,    45,    45,    45,
      45,     3,    62,    49,    16,    37,    44,    44,     9,    10,
      12,    13,    53,    54,    13,     8,     9,    49,    49,    44,
      49,    49,    49,     9,    13,    13,    13,    17,    13,     0,
       3,    33,    35,    63,    16,    37,    45,    45,    45,    45,
      12,    13,    12,    13,    45,    45,    44,    44,    44,    44,
      45,    45,    49,    45,    45,    13,    17,    38,    12,    13,
       8,     9,    53,    54,    13,    13,    16,    37,    13,    17,
      38,    45,    45,    45,    45,     9,    53,    54,    13,    17,
      38,    45
};

  /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_int8 yyr1[] =
{
       0,    39,    40,    40,    41,    41,    42,    42,    42,    43,
      43,    43,    43,    43,    44,    44,    44,    45,    45,    46,
      47,    47,    47,    48,    48,    48,    48,    48,    48,    49,
      49,    50,    50,    50,    51,    51,    51,    51,    51,    52,
      52,    52,    52,    52,    52,    52,    52,    52,    52,    52,
      53,    53,    53,    53,    54,    54,    54,    54,    54,    54,
      55,    55,    55,    55,    56,    56,    56,    57,    57,    58,
      58,    58,    58,    58,    58,    58,    58,    58,    58,    59,
      59,    60,    60,    61,    61,    62,    62,    63,    63,    63,
      64,    64,    64,    64,    64,    64,    64,    64,    64,    64,
      64,    64,    64,    64,    64,    64,    64,    64,    64,    64,
      64,    64,    64,    64
};

  /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static const yytype_int8 yyr2[] =
{
       0,     2,     2,     1,     1,     1,     1,     2,     2,     4,
       4,     3,     3,     1,     3,     3,     1,     1,     2,     2,
       3,     3,     3,     4,     4,     4,     4,     3,     1,     0,
       2,     1,     1,     2,     1,     1,     1,     1,     1,     3,
       5,     4,     5,     5,     7,     7,     6,     6,     6,     6,
       4,     6,     5,     5,     2,     4,     4,     6,     5,     5,
       3,     5,     5,     7,     3,     5,     5,     3,     5,     6,
       6,     6,     7,     7,     7,     7,     9,     9,     9,     1,
       2,     1,     1,     1,     2,     1,     2,     1,     1,     1,
       2,     3,     2,     2,     3,     3,     3,     4,     2,     3,
       2,     2,     3,     3,     3,     4,     2,     2,     2,     3,
       2,     2,     2,     3
};


enum { YYENOMEM = -2 };

#define yyerrok         (yyerrstatus = 0)
#define yyclearin       (yychar = YYEMPTY)

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab


#define YYRECOVERING()  (!!yyerrstatus)

#define YYBACKUP(Token, Value)                                    \
  do                                                              \
    if (yychar == YYEMPTY)                                        \
      {                                                           \
        yychar = (Token);                                         \
        yylval = (Value);                                         \
        YYPOPSTACK (yylen);                                       \
        yystate = *yyssp;                                         \
        goto yybackup;                                            \
      }                                                           \
    else                                                          \
      {                                                           \
        yyerror (p, YY_("syntax error: cannot back up")); \
        YYERROR;                                                  \
      }                                                           \
  while (0)

/* Backward compatibility with an undocumented macro.
   Use YYerror or YYUNDEF. */
#define YYERRCODE YYUNDEF


/* Enable debugging if requested.  */
#if YYDEBUG

# ifndef YYFPRINTF
#  include <stdio.h> /* INFRINGES ON USER NAME SPACE */
#  define YYFPRINTF fprintf
# endif

# define YYDPRINTF(Args)                        \
do {                                            \
  if (yydebug)                                  \
    YYFPRINTF Args;                             \
} while (0)

/* This macro is provided for backward compatibility. */
# ifndef YY_LOCATION_PRINT
#  define YY_LOCATION_PRINT(File, Loc) ((void) 0)
# endif


# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)                    \
do {                                                                      \
  if (yydebug)                                                            \
    {                                                                     \
      YYFPRINTF (stderr, "%s ", Title);                                   \
      yy_symbol_print (stderr,                                            \
                  Kind, Value, p); \
      YYFPRINTF (stderr, "\n");                                           \
    }                                                                     \
} while (0)


/*-----------------------------------.
| Print this symbol's value on YYO.  |
`-----------------------------------*/

static void
yy_symbol_value_print (FILE *yyo,
                       yysymbol_kind_t yykind, YYSTYPE const * const yyvaluep, parser_state* p)
{
  FILE *yyoutput = yyo;
  YYUSE (yyoutput);
  YYUSE (p);
  if (!yyvaluep)
    return;
# ifdef YYPRINT
  if (yykind < YYNTOKENS)
    YYPRINT (yyo, yytoknum[yykind], *yyvaluep);
# endif
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/*---------------------------.
| Print this symbol on YYO.  |
`---------------------------*/

static void
yy_symbol_print (FILE *yyo,
                 yysymbol_kind_t yykind, YYSTYPE const * const yyvaluep, parser_state* p)
{
  YYFPRINTF (yyo, "%s %s (",
             yykind < YYNTOKENS ? "token" : "nterm", yysymbol_name (yykind));

  yy_symbol_value_print (yyo, yykind, yyvaluep, p);
  YYFPRINTF (yyo, ")");
}

/*------------------------------------------------------------------.
| yy_stack_print -- Print the state stack from its BOTTOM up to its |
| TOP (included).                                                   |
`------------------------------------------------------------------*/

static void
yy_stack_print (yy_state_t *yybottom, yy_state_t *yytop)
{
  YYFPRINTF (stderr, "Stack now");
  for (; yybottom <= yytop; yybottom++)
    {
      int yybot = *yybottom;
      YYFPRINTF (stderr, " %d", yybot);
    }
  YYFPRINTF (stderr, "\n");
}

# define YY_STACK_PRINT(Bottom, Top)                            \
do {                                                            \
  if (yydebug)                                                  \
    yy_stack_print ((Bottom), (Top));                           \
} while (0)


/*------------------------------------------------.
| Report that the YYRULE is going to be reduced.  |
`------------------------------------------------*/

static void
yy_reduce_print (yy_state_t *yyssp, YYSTYPE *yyvsp,
                 int yyrule, parser_state* p)
{
  int yylno = yyrline[yyrule];
  int yynrhs = yyr2[yyrule];
  int yyi;
  YYFPRINTF (stderr, "Reducing stack by rule %d (line %d):\n",
             yyrule - 1, yylno);
  /* The symbols being reduced.  */
  for (yyi = 0; yyi < yynrhs; yyi++)
    {
      YYFPRINTF (stderr, "   $%d = ", yyi + 1);
      yy_symbol_print (stderr,
                       YY_ACCESSING_SYMBOL (+yyssp[yyi + 1 - yynrhs]),
                       &yyvsp[(yyi + 1) - (yynrhs)], p);
      YYFPRINTF (stderr, "\n");
    }
}

# define YY_REDUCE_PRINT(Rule)          \
do {                                    \
  if (yydebug)                          \
    yy_reduce_print (yyssp, yyvsp, Rule, p); \
} while (0)

/* Nonzero means print parse trace.  It is left uninitialized so that
   multiple parsers can coexist.  */
int yydebug;
#else /* !YYDEBUG */
# define YYDPRINTF(Args) ((void) 0)
# define YY_SYMBOL_PRINT(Title, Kind, Value, Location)
# define YY_STACK_PRINT(Bottom, Top)
# define YY_REDUCE_PRINT(Rule)
#endif /* !YYDEBUG */


/* YYINITDEPTH -- initial size of the parser's stacks.  */
#ifndef YYINITDEPTH
# define YYINITDEPTH 200
#endif

/* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
   evaluated with infinite-precision integer arithmetic.  */

#ifndef YYMAXDEPTH
# define YYMAXDEPTH 10000
#endif






/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/

static void
yydestruct (const char *yymsg,
            yysymbol_kind_t yykind, YYSTYPE *yyvaluep, parser_state* p)
{
  YYUSE (yyvaluep);
  YYUSE (p);
  if (!yymsg)
    yymsg = "Deleting";
  YY_SYMBOL_PRINT (yymsg, yykind, yyvaluep, yylocationp);

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yykind);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}






/*----------.
| yyparse.  |
`----------*/

int
yyparse (parser_state* p)
{
/* Lookahead token kind.  */
int yychar;


/* The semantic value of the lookahead symbol.  */
/* Default value used for initialization, for pacifying older GCCs
   or non-GCC compilers.  */
YY_INITIAL_VALUE (static YYSTYPE yyval_default;)
YYSTYPE yylval YY_INITIAL_VALUE (= yyval_default);

    /* Number of syntax errors so far.  */
    int yynerrs = 0;

    yy_state_fast_t yystate = 0;
    /* Number of tokens to shift before error messages enabled.  */
    int yyerrstatus = 0;

    /* Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */

    /* Their size.  */
    YYPTRDIFF_T yystacksize = YYINITDEPTH;

    /* The state stack: array, bottom, top.  */
    yy_state_t yyssa[YYINITDEPTH];
    yy_state_t *yyss = yyssa;
    yy_state_t *yyssp = yyss;

    /* The semantic value stack: array, bottom, top.  */
    YYSTYPE yyvsa[YYINITDEPTH];
    YYSTYPE *yyvs = yyvsa;
    YYSTYPE *yyvsp = yyvs;

  int yyn;
  /* The return value of yyparse.  */
  int yyresult;
  /* Lookahead symbol kind.  */
  yysymbol_kind_t yytoken = YYSYMBOL_YYEMPTY;
  /* The variables used to return semantic value and location from the
     action routines.  */
  YYSTYPE yyval;



#define YYPOPSTACK(N)   (yyvsp -= (N), yyssp -= (N))

  /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
  int yylen = 0;

  YYDPRINTF ((stderr, "Starting parse\n"));

  yychar = YYEMPTY; /* Cause a token to be read.  */
  goto yysetstate;


/*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
yynewstate:
  /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
  yyssp++;


/*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
yysetstate:
  YYDPRINTF ((stderr, "Entering state %d\n", yystate));
  YY_ASSERT (0 <= yystate && yystate < YYNSTATES);
  YY_IGNORE_USELESS_CAST_BEGIN
  *yyssp = YY_CAST (yy_state_t, yystate);
  YY_IGNORE_USELESS_CAST_END
  YY_STACK_PRINT (yyss, yyssp);

  if (yyss + yystacksize - 1 <= yyssp)
#if !defined yyoverflow && !defined YYSTACK_RELOCATE
    goto yyexhaustedlab;
#else
    {
      /* Get the current used size of the three stacks, in elements.  */
      YYPTRDIFF_T yysize = yyssp - yyss + 1;

# if defined yyoverflow
      {
        /* Give user a chance to reallocate the stack.  Use copies of
           these so that the &'s don't force the real ones into
           memory.  */
        yy_state_t *yyss1 = yyss;
        YYSTYPE *yyvs1 = yyvs;

        /* Each stack pointer address is followed by the size of the
           data in use in that stack, in bytes.  This used to be a
           conditional around just the two extra args, but that might
           be undefined if yyoverflow is a macro.  */
        yyoverflow (YY_("memory exhausted"),
                    &yyss1, yysize * YYSIZEOF (*yyssp),
                    &yyvs1, yysize * YYSIZEOF (*yyvsp),
                    &yystacksize);
        yyss = yyss1;
        yyvs = yyvs1;
      }
# else /* defined YYSTACK_RELOCATE */
      /* Extend the stack our own way.  */
      if (YYMAXDEPTH <= yystacksize)
        goto yyexhaustedlab;
      yystacksize *= 2;
      if (YYMAXDEPTH < yystacksize)
        yystacksize = YYMAXDEPTH;

      {
        yy_state_t *yyss1 = yyss;
        union yyalloc *yyptr =
          YY_CAST (union yyalloc *,
                   YYSTACK_ALLOC (YY_CAST (YYSIZE_T, YYSTACK_BYTES (yystacksize))));
        if (! yyptr)
          goto yyexhaustedlab;
        YYSTACK_RELOCATE (yyss_alloc, yyss);
        YYSTACK_RELOCATE (yyvs_alloc, yyvs);
#  undef YYSTACK_RELOCATE
        if (yyss1 != yyssa)
          YYSTACK_FREE (yyss1);
      }
# endif

      yyssp = yyss + yysize - 1;
      yyvsp = yyvs + yysize - 1;

      YY_IGNORE_USELESS_CAST_BEGIN
      YYDPRINTF ((stderr, "Stack size increased to %ld\n",
                  YY_CAST (long, yystacksize)));
      YY_IGNORE_USELESS_CAST_END

      if (yyss + yystacksize - 1 <= yyssp)
        YYABORT;
    }
#endif /* !defined yyoverflow && !defined YYSTACK_RELOCATE */

  if (yystate == YYFINAL)
    YYACCEPT;

  goto yybackup;


/*-----------.
| yybackup.  |
`-----------*/
yybackup:
  /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */

  /* First try to decide what to do without reference to lookahead token.  */
  yyn = yypact[yystate];
  if (yypact_value_is_default (yyn))
    goto yydefault;

  /* Not known => get a lookahead token if don't already have one.  */

  /* YYCHAR is either empty, or end-of-input, or a valid lookahead.  */
  if (yychar == YYEMPTY)
    {
      YYDPRINTF ((stderr, "Reading a token\n"));
      yychar = yylex (&yylval, p);
    }

  if (yychar <= YYEOF)
    {
      yychar = YYEOF;
      yytoken = YYSYMBOL_YYEOF;
      YYDPRINTF ((stderr, "Now at end of input.\n"));
    }
  else if (yychar == YYerror)
    {
      /* The scanner already issued an error message, process directly
         to error recovery.  But do not keep the error token as
         lookahead, it is too special and may lead us to an endless
         loop in error recovery. */
      yychar = YYUNDEF;
      yytoken = YYSYMBOL_YYerror;
      goto yyerrlab1;
    }
  else
    {
      yytoken = YYTRANSLATE (yychar);
      YY_SYMBOL_PRINT ("Next token is", yytoken, &yylval, &yylloc);
    }

  /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
  yyn += yytoken;
  if (yyn < 0 || YYLAST < yyn || yycheck[yyn] != yytoken)
    goto yydefault;
  yyn = yytable[yyn];
  if (yyn <= 0)
    {
      if (yytable_value_is_error (yyn))
        goto yyerrlab;
      yyn = -yyn;
      goto yyreduce;
    }

  /* Count tokens shifted since error; after three, turn off error
     status.  */
  if (yyerrstatus)
    yyerrstatus--;

  /* Shift the lookahead token.  */
  YY_SYMBOL_PRINT ("Shifting", yytoken, &yylval, &yylloc);
  yystate = yyn;
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END

  /* Discard the shifted token.  */
  yychar = YYEMPTY;
  goto yynewstate;


/*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
yydefault:
  yyn = yydefact[yystate];
  if (yyn == 0)
    goto yyerrlab;
  goto yyreduce;


/*-----------------------------.
| yyreduce -- do a reduction.  |
`-----------------------------*/
yyreduce:
  /* yyn is the number of a rule to reduce with.  */
  yylen = yyr2[yyn];

  /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
  yyval = yyvsp[1-yylen];


  YY_REDUCE_PRINT (yyn);
  switch (yyn)
    {
  case 2: /* inputunit: simple_list terminator  */
#line 57 "mrbgems/mruby-reddish-parser/core/parser.y"
                         { p->result = yyvsp[-1]; YYACCEPT;}
#line 1378 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 3: /* inputunit: $end  */
#line 58 "mrbgems/mruby-reddish-parser/core/parser.y"
                         { p->result = NIL;YYACCEPT;}
#line 1384 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 7: /* simple_list: connector '&'  */
#line 64 "mrbgems/mruby-reddish-parser/core/parser.y"
                { yyval = ASYNC(p, yyvsp[-1]); }
#line 1390 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 9: /* connector: connector AND_AND newline_list pipeline  */
#line 68 "mrbgems/mruby-reddish-parser/core/parser.y"
                                          { yyval = CONNECTOR(p, "and", yyvsp[-3], yyvsp[0]); }
#line 1396 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 10: /* connector: connector OR_OR newline_list pipeline  */
#line 69 "mrbgems/mruby-reddish-parser/core/parser.y"
                                          { yyval = CONNECTOR(p, "or",  yyvsp[-3], yyvsp[0]); }
#line 1402 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 11: /* connector: connector '&' pipeline  */
#line 70 "mrbgems/mruby-reddish-parser/core/parser.y"
                                          { yyval = CONNECTOR(p, "async", yyvsp[-2], yyvsp[0]); }
#line 1408 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 12: /* connector: connector ';' pipeline  */
#line 71 "mrbgems/mruby-reddish-parser/core/parser.y"
                                          { yyval = CONNECTOR(p, "semicolon", yyvsp[-2], yyvsp[0]); }
#line 1414 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 14: /* pipeline: pipeline '|' command  */
#line 75 "mrbgems/mruby-reddish-parser/core/parser.y"
                          { yyval = PIPELINE(p, yyvsp[-2], yyvsp[0], MRB_FALSE); }
#line 1420 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 15: /* pipeline: pipeline OR_AND command  */
#line 76 "mrbgems/mruby-reddish-parser/core/parser.y"
                          { yyval = PIPELINE(p, yyvsp[-2], yyvsp[0], MRB_TRUE); }
#line 1426 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 18: /* compound_list: newline_list list1  */
#line 81 "mrbgems/mruby-reddish-parser/core/parser.y"
                     { yyval = yyvsp[0]; }
#line 1432 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 19: /* list: newline_list list0  */
#line 84 "mrbgems/mruby-reddish-parser/core/parser.y"
                     { yyval = yyvsp[0]; }
#line 1438 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 21: /* list0: list1 '&' newline_list  */
#line 88 "mrbgems/mruby-reddish-parser/core/parser.y"
                          { yyval = ASYNC(p, yyvsp[-2]); }
#line 1444 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 23: /* list1: list1 AND_AND newline_list pipeline  */
#line 92 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "and", yyvsp[-3], yyvsp[0]); }
#line 1450 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 24: /* list1: list1 OR_OR newline_list pipeline  */
#line 93 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "or",  yyvsp[-3], yyvsp[0]); }
#line 1456 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 25: /* list1: list1 '&' newline_list pipeline  */
#line 94 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "async", yyvsp[-3], yyvsp[0]); }
#line 1462 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 26: /* list1: list1 ';' newline_list pipeline  */
#line 95 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "semicolon", yyvsp[-3], yyvsp[0]); }
#line 1468 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 27: /* list1: list1 '\n' pipeline  */
#line 96 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "semicolon", yyvsp[-2], yyvsp[0]); }
#line 1474 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 31: /* command: simple_command  */
#line 104 "mrbgems/mruby-reddish-parser/core/parser.y"
                 { yyval = COMMAND(p, yyvsp[0]); }
#line 1480 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 33: /* command: shell_command redirect_list  */
#line 106 "mrbgems/mruby-reddish-parser/core/parser.y"
                              { APPEND_REDIRECT(p, yyvsp[-1], yyvsp[0]); yyval = yyvsp[-1]; }
#line 1486 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 39: /* if_statement: IF compound_list END  */
#line 116 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-1], 0, NIL); }
#line 1492 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 40: /* if_statement: IF compound_list ELSE compound_list END  */
#line 117 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 2, NIL, yyvsp[-1]); }
#line 1498 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 41: /* if_statement: IF compound_list elsif_clause END  */
#line 118 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-2], 2, NIL, yyvsp[-1]); }
#line 1504 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 42: /* if_statement: IF compound_list THEN compound_list FI  */
#line 119 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 1, yyvsp[-1]); }
#line 1510 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 43: /* if_statement: IF compound_list THEN compound_list END  */
#line 120 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 1, yyvsp[-1]); }
#line 1516 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 44: /* if_statement: IF compound_list THEN compound_list ELSE compound_list FI  */
#line 121 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-5], 2, yyvsp[-3], yyvsp[-1]); }
#line 1522 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 45: /* if_statement: IF compound_list THEN compound_list ELSE compound_list END  */
#line 122 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-5], 2, yyvsp[-3], yyvsp[-1]); }
#line 1528 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 46: /* if_statement: IF compound_list THEN compound_list elif_clause FI  */
#line 123 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[-1]); }
#line 1534 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 47: /* if_statement: IF compound_list THEN compound_list elif_clause END  */
#line 124 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[-1]); }
#line 1540 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 48: /* if_statement: IF compound_list THEN compound_list elsif_clause FI  */
#line 125 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[-1]); }
#line 1546 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 49: /* if_statement: IF compound_list THEN compound_list elsif_clause END  */
#line 126 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[-1]); }
#line 1552 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 50: /* elif_clause: ELIF compound_list THEN compound_list  */
#line 129 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                           { yyval = IF_STMT(p, yyvsp[-2], 1, yyvsp[0]); }
#line 1558 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 51: /* elif_clause: ELIF compound_list THEN compound_list ELSE compound_list  */
#line 130 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                           { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[0]); }
#line 1564 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 52: /* elif_clause: ELIF compound_list THEN compound_list elif_clause  */
#line 131 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                           { yyval = IF_STMT(p, yyvsp[-3], 2, yyvsp[-1], yyvsp[0]); }
#line 1570 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 53: /* elif_clause: ELIF compound_list THEN compound_list elsif_clause  */
#line 132 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                           { yyval = IF_STMT(p, yyvsp[-3], 2, yyvsp[-1], yyvsp[0]); }
#line 1576 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 54: /* elsif_clause: ELSIF compound_list  */
#line 135 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[0], 0, NIL); }
#line 1582 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 55: /* elsif_clause: ELSIF compound_list ELSE compound_list  */
#line 136 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-2], 2, NIL, yyvsp[0]); }
#line 1588 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 56: /* elsif_clause: ELSIF compound_list THEN compound_list  */
#line 137 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-2], 1, yyvsp[0]); }
#line 1594 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 57: /* elsif_clause: ELSIF compound_list THEN compound_list ELSE compound_list  */
#line 138 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[0]); }
#line 1600 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 58: /* elsif_clause: ELSIF compound_list THEN compound_list elif_clause  */
#line 139 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 2, yyvsp[-1], yyvsp[0]); }
#line 1606 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 59: /* elsif_clause: ELSIF compound_list THEN compound_list elsif_clause  */
#line 140 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 2, yyvsp[-1], yyvsp[0]); }
#line 1612 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 60: /* unless_statement: UNLESS compound_list END  */
#line 143 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                 { yyval = UNLESS_STMT(p, yyvsp[-1], 0, NIL); }
#line 1618 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 61: /* unless_statement: UNLESS compound_list ELSE compound_list END  */
#line 144 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                 { yyval = UNLESS_STMT(p, yyvsp[-3], 2, NIL, yyvsp[-1]); }
#line 1624 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 62: /* unless_statement: UNLESS compound_list THEN compound_list END  */
#line 145 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                 { yyval = UNLESS_STMT(p, yyvsp[-3], 1, yyvsp[-1]); }
#line 1630 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 63: /* unless_statement: UNLESS compound_list THEN compound_list ELSE compound_list END  */
#line 146 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                 { yyval = UNLESS_STMT(p, yyvsp[-5], 2, yyvsp[-3], yyvsp[-1]); }
#line 1636 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 64: /* while_statement: WHILE compound_list END  */
#line 149 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = WHILE_STMT(p, yyvsp[-1], NIL); }
#line 1642 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 65: /* while_statement: WHILE compound_list DO compound_list DONE  */
#line 150 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = WHILE_STMT(p, yyvsp[-3], yyvsp[-1]); }
#line 1648 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 66: /* while_statement: WHILE compound_list DO compound_list END  */
#line 151 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = WHILE_STMT(p, yyvsp[-3], yyvsp[-1]); }
#line 1654 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 67: /* until_statement: UNTIL compound_list END  */
#line 154 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = UNTIL_STMT(p, yyvsp[-1], NIL); }
#line 1660 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 68: /* until_statement: UNTIL compound_list DO compound_list END  */
#line 155 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = UNTIL_STMT(p, yyvsp[-3], yyvsp[-1]); }
#line 1666 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 69: /* for_statement: FOR WORD newline_list DO compound_list DONE  */
#line 158 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-4], NIL, yyvsp[-1]); }
#line 1672 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 70: /* for_statement: FOR WORD newline_list DO compound_list END  */
#line 159 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-4], NIL, yyvsp[-1]); }
#line 1678 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 71: /* for_statement: FOR WORD newline_list '{' compound_list '}'  */
#line 160 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-4], NIL, yyvsp[-1]); }
#line 1684 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 72: /* for_statement: FOR WORD ';' newline_list DO compound_list DONE  */
#line 161 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-5], NIL, yyvsp[-1]); }
#line 1690 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 73: /* for_statement: FOR WORD ';' newline_list DO compound_list END  */
#line 162 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-5], NIL, yyvsp[-1]); }
#line 1696 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 74: /* for_statement: FOR WORD ';' newline_list '{' compound_list '}'  */
#line 163 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-5], NIL, yyvsp[-1]); }
#line 1702 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 75: /* for_statement: FOR WORD IN word_list list_terminater compound_list END  */
#line 164 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                              { yyval = FOR_STMT(p, yyvsp[-5], yyvsp[-3], yyvsp[-1]); }
#line 1708 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 76: /* for_statement: FOR WORD IN word_list list_terminater newline_list DO compound_list DONE  */
#line 165 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                           { yyval = FOR_STMT(p, yyvsp[-7], yyvsp[-5], yyvsp[-1]); }
#line 1714 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 77: /* for_statement: FOR WORD IN word_list list_terminater newline_list DO compound_list END  */
#line 166 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                           { yyval = FOR_STMT(p, yyvsp[-7], yyvsp[-5], yyvsp[-1]); }
#line 1720 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 78: /* for_statement: FOR WORD IN word_list list_terminater newline_list '{' compound_list '}'  */
#line 167 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                           { yyval = FOR_STMT(p, yyvsp[-7], yyvsp[-5], yyvsp[-1]); }
#line 1726 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 79: /* simple_command: simple_command_element  */
#line 170 "mrbgems/mruby-reddish-parser/core/parser.y"
                         { yyval = mrb_ary_new_from_values(p->state, 1, &yyvsp[0]); }
#line 1732 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 80: /* simple_command: simple_command simple_command_element  */
#line 171 "mrbgems/mruby-reddish-parser/core/parser.y"
                                        { mrb_ary_push(p->state, yyvsp[-1], yyvsp[0]); yyval = yyvsp[-1]; }
#line 1738 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 81: /* simple_command_element: WORD  */
#line 174 "mrbgems/mruby-reddish-parser/core/parser.y"
       { yyval = WORD(p, yyvsp[0]); }
#line 1744 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 84: /* redirect_list: redirect_list redirect  */
#line 179 "mrbgems/mruby-reddish-parser/core/parser.y"
                         { mrb_ary_concat(p->state, yyvsp[-1], yyvsp[0]); yyval = yyvsp[-1]; }
#line 1750 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 85: /* word_list: WORD  */
#line 182 "mrbgems/mruby-reddish-parser/core/parser.y"
       { yyval = mrb_ary_new_from_values(p->state, 1, &yyvsp[0]); }
#line 1756 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 86: /* word_list: word_list WORD  */
#line 183 "mrbgems/mruby-reddish-parser/core/parser.y"
                 { mrb_ary_push(p->state, yyvsp[-1], yyvsp[0]); yyval = yyvsp[-1]; }
#line 1762 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 90: /* redirect: LT WORD  */
#line 188 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "read",     2, FIXNUM(0), yyvsp[0]); }
#line 1768 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 91: /* redirect: NUMBER LT WORD  */
#line 189 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "read",     2, yyvsp[-2], yyvsp[0]); }
#line 1774 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 92: /* redirect: LT_AND MINUS  */
#line 190 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "close",    1, FIXNUM(0)); }
#line 1780 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 93: /* redirect: LT_AND NUMBER  */
#line 191 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyread", 2, FIXNUM(0), yyvsp[0]); }
#line 1786 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 94: /* redirect: LT_AND NUMBER MINUS  */
#line 192 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyreadclose", 2, FIXNUM(0), yyvsp[-1]); }
#line 1792 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 95: /* redirect: NUMBER LT_AND MINUS  */
#line 193 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "close",    1, yyvsp[-2]); }
#line 1798 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 96: /* redirect: NUMBER LT_AND NUMBER  */
#line 194 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyread", 2, yyvsp[-2], yyvsp[0]); }
#line 1804 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 97: /* redirect: NUMBER LT_AND NUMBER MINUS  */
#line 195 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyreadclose", 2, yyvsp[-3], yyvsp[-1]); }
#line 1810 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 98: /* redirect: GT WORD  */
#line 196 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "write",     2, FIXNUM(1), yyvsp[0]); }
#line 1816 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 99: /* redirect: NUMBER GT WORD  */
#line 197 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "write",     2, yyvsp[-2], yyvsp[0]); }
#line 1822 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 100: /* redirect: GT_AND MINUS  */
#line 198 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "close",     1, FIXNUM(1)); }
#line 1828 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 101: /* redirect: GT_AND NUMBER  */
#line 199 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copywrite", 2, FIXNUM(1), yyvsp[0]); }
#line 1834 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 102: /* redirect: GT_AND NUMBER MINUS  */
#line 200 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copywriteclose", 2, FIXNUM(1), yyvsp[-1]); }
#line 1840 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 103: /* redirect: NUMBER GT_AND MINUS  */
#line 201 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "close",     1, yyvsp[-2]); }
#line 1846 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 104: /* redirect: NUMBER GT_AND NUMBER  */
#line 202 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copywrite", 2, yyvsp[-2], yyvsp[0]); }
#line 1852 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 105: /* redirect: NUMBER GT_AND NUMBER MINUS  */
#line 203 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copywriteclose", 2, yyvsp[-3], yyvsp[-1]); }
#line 1858 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 106: /* redirect: AND_GT WORD  */
#line 204 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copystdoutstderr", 3, FIXNUM(1), FIXNUM(2), yyvsp[0]); }
#line 1864 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 107: /* redirect: GT_AND WORD  */
#line 205 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copystdoutstderr", 3, FIXNUM(1), FIXNUM(2), yyvsp[0]); }
#line 1870 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 108: /* redirect: GT_GT WORD  */
#line 206 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "append", 2, FIXNUM(1), yyvsp[0]); }
#line 1876 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 109: /* redirect: NUMBER GT_GT WORD  */
#line 207 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "append", 2, yyvsp[-2], yyvsp[0]); }
#line 1882 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 110: /* redirect: AND_GT_GT WORD  */
#line 208 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyappend", 2, FIXNUM(1), yyvsp[0]); }
#line 1888 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 111: /* redirect: GT_GT_AND WORD  */
#line 209 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyappend", 2, FIXNUM(1), yyvsp[0]); }
#line 1894 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 112: /* redirect: LT_GT WORD  */
#line 210 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "readwrite", 2, FIXNUM(0), yyvsp[0]); }
#line 1900 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 113: /* redirect: NUMBER LT_GT WORD  */
#line 211 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "readwrite", 2, yyvsp[-2], yyvsp[0]); }
#line 1906 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;


#line 1910 "mrbgems/mruby-reddish-parser/src/parser.c"

      default: break;
    }
  /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
  YY_SYMBOL_PRINT ("-> $$ =", YY_CAST (yysymbol_kind_t, yyr1[yyn]), &yyval, &yyloc);

  YYPOPSTACK (yylen);
  yylen = 0;

  *++yyvsp = yyval;

  /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
  {
    const int yylhs = yyr1[yyn] - YYNTOKENS;
    const int yyi = yypgoto[yylhs] + *yyssp;
    yystate = (0 <= yyi && yyi <= YYLAST && yycheck[yyi] == *yyssp
               ? yytable[yyi]
               : yydefgoto[yylhs]);
  }

  goto yynewstate;


/*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
yyerrlab:
  /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
  yytoken = yychar == YYEMPTY ? YYSYMBOL_YYEMPTY : YYTRANSLATE (yychar);
  /* If not already recovering from an error, report this error.  */
  if (!yyerrstatus)
    {
      ++yynerrs;
      yyerror (p, YY_("syntax error"));
    }

  if (yyerrstatus == 3)
    {
      /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */

      if (yychar <= YYEOF)
        {
          /* Return failure if at end of input.  */
          if (yychar == YYEOF)
            YYABORT;
        }
      else
        {
          yydestruct ("Error: discarding",
                      yytoken, &yylval, p);
          yychar = YYEMPTY;
        }
    }

  /* Else will try to reuse lookahead token after shifting the error
     token.  */
  goto yyerrlab1;


/*---------------------------------------------------.
| yyerrorlab -- error raised explicitly by YYERROR.  |
`---------------------------------------------------*/
yyerrorlab:
  /* Pacify compilers when the user code never invokes YYERROR and the
     label yyerrorlab therefore never appears in user code.  */
  if (0)
    YYERROR;

  /* Do not reclaim the symbols of the rule whose action triggered
     this YYERROR.  */
  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);
  yystate = *yyssp;
  goto yyerrlab1;


/*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
yyerrlab1:
  yyerrstatus = 3;      /* Each real token shifted decrements this.  */

  /* Pop stack until we find a state that shifts the error token.  */
  for (;;)
    {
      yyn = yypact[yystate];
      if (!yypact_value_is_default (yyn))
        {
          yyn += YYSYMBOL_YYerror;
          if (0 <= yyn && yyn <= YYLAST && yycheck[yyn] == YYSYMBOL_YYerror)
            {
              yyn = yytable[yyn];
              if (0 < yyn)
                break;
            }
        }

      /* Pop the current state because it cannot handle the error token.  */
      if (yyssp == yyss)
        YYABORT;


      yydestruct ("Error: popping",
                  YY_ACCESSING_SYMBOL (yystate), yyvsp, p);
      YYPOPSTACK (1);
      yystate = *yyssp;
      YY_STACK_PRINT (yyss, yyssp);
    }

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END


  /* Shift the error token.  */
  YY_SYMBOL_PRINT ("Shifting", YY_ACCESSING_SYMBOL (yyn), yyvsp, yylsp);

  yystate = yyn;
  goto yynewstate;


/*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
yyacceptlab:
  yyresult = 0;
  goto yyreturn;


/*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
yyabortlab:
  yyresult = 1;
  goto yyreturn;


#if !defined yyoverflow
/*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
yyexhaustedlab:
  yyerror (p, YY_("memory exhausted"));
  yyresult = 2;
  goto yyreturn;
#endif


/*-------------------------------------------------------.
| yyreturn -- parsing is finished, clean up and return.  |
`-------------------------------------------------------*/
yyreturn:
  if (yychar != YYEMPTY)
    {
      /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
      yytoken = YYTRANSLATE (yychar);
      yydestruct ("Cleanup: discarding lookahead",
                  yytoken, &yylval, p);
    }
  /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
  YYPOPSTACK (yylen);
  YY_STACK_PRINT (yyss, yyssp);
  while (yyssp != yyss)
    {
      yydestruct ("Cleanup: popping",
                  YY_ACCESSING_SYMBOL (+*yyssp), yyvsp, p);
      YYPOPSTACK (1);
    }
#ifndef yyoverflow
  if (yyss != yyssa)
    YYSTACK_FREE (yyss);
#endif

  return yyresult;
}

#line 213 "mrbgems/mruby-reddish-parser/core/parser.y"

static const struct token_type {
    const char *name;
    int type;
} token_types[] = {
    {"\n", '\n'},
    {";",  ';'},
    {"&",  '&'},
    {"|",  '|'},
    {"{",  '{'},
    {"}",  '}'},
    {"&&", AND_AND},
    {"&>", AND_GT},
    {"&>>", AND_GT_GT},
    {">",  GT},
    {">&", GT_AND},
    {">>&", GT_GT_AND},
    {">>", GT_GT},
    {"<",  LT},
    {"<&", LT_AND},
    {"<>", LT_GT},
    {"-",  MINUS},
    {"|&", OR_AND},
    {"||", OR_OR},
    {"eof", YYEOF},
    {"number", NUMBER},
    {"word", WORD},
    {"if", IF},
    {"then", THEN},
    {"else", ELSE},
    {"elif", ELIF},
    {"elsif", ELSIF},
    {"fi", FI},
    {"end", END},
    {"unless", UNLESS},
    {"while", WHILE},
    {"do", DO},
    {"done", DONE},
    {"until", UNTIL},
    {"for", FOR},
    {"in", IN},
    {NULL, 0},
};

int sym2tt(const char *sym) {
    const struct token_type *tt;
    for (tt = token_types; tt->name; tt++) {
        if (strcmp(tt->name, sym) == 0) {
            return tt->type;
        }
    }
    return 0;
}

int yylex(void* lval, parser_state* p) {
    mrb_value token, token_type;
    int type;

    token = mrb_funcall(p->state, p->lexer, "get_token", 0);
    token_type = mrb_funcall(p->state, token, "type", 0);
    type = sym2tt(mrb_sym2name(p->state, mrb_symbol(token_type)));
    *((YYSTYPE*)lval) = mrb_funcall(p->state, token, "data", 0);

    return type;
}

void yyerror(parser_state* p, const char* s){
    mrb_value str = mrb_str_new_cstr(p->state, s);
    mrb_value state = mrb_funcall(p->state, p->lexer, "state", 0);
    mrb_funcall(p->state, p->action, "on_error", 2, str, state);
}


mrb_value mrb_reddish_parser_parse(mrb_state *mrb, mrb_value self) {
    mrb_value line;
    mrb_value action, lexer;
    struct RClass *action_class;
    struct RClass *lexer_class;
    parser_state pstate;

    mrb_get_args(mrb, "S", &line);

    action_class = mrb_class_get_under(mrb, mrb_module_get(mrb, "ReddishParser"), "Action");
    action = mrb_obj_new(mrb, action_class, 0, NULL);

    lexer_class = mrb_class_get_under(mrb, mrb_module_get(mrb, "ReddishParser"), "Lexer");
    lexer = mrb_obj_new(mrb, lexer_class, 1, &line);

    pstate.state = mrb;
    pstate.result = mrb_nil_value();
    pstate.action = action;
    pstate.lexer = lexer;

    yyparse(&pstate);

    if (mrb_nil_p(pstate.result)) {
        return mrb_nil_value();
    }

    return pstate.result;
}

mrb_value mrb_reddish_parser_debug(mrb_state *mrb, mrb_value self) {
    mrb_bool flag;
    mrb_get_args(mrb, "b", &flag);
    yydebug = flag;
    return mrb_bool_value(flag);
}
