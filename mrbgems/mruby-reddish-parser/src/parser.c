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
    GT_AND = 282,                  /* GT_AND  */
    LT = 283,                      /* LT  */
    LT_AND = 284,                  /* LT_AND  */
    LT_GT = 285                    /* LT_GT  */
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
  YYSYMBOL_GT_AND = 27,                    /* GT_AND  */
  YYSYMBOL_LT = 28,                        /* LT  */
  YYSYMBOL_LT_AND = 29,                    /* LT_AND  */
  YYSYMBOL_LT_GT = 30,                     /* LT_GT  */
  YYSYMBOL_31_n_ = 31,                     /* '\n'  */
  YYSYMBOL_32_ = 32,                       /* '&'  */
  YYSYMBOL_33_ = 33,                       /* ';'  */
  YYSYMBOL_34_ = 34,                       /* '|'  */
  YYSYMBOL_35_ = 35,                       /* '{'  */
  YYSYMBOL_36_ = 36,                       /* '}'  */
  YYSYMBOL_YYACCEPT = 37,                  /* $accept  */
  YYSYMBOL_inputunit = 38,                 /* inputunit  */
  YYSYMBOL_terminator = 39,                /* terminator  */
  YYSYMBOL_simple_list = 40,               /* simple_list  */
  YYSYMBOL_connector = 41,                 /* connector  */
  YYSYMBOL_pipeline = 42,                  /* pipeline  */
  YYSYMBOL_compound_list = 43,             /* compound_list  */
  YYSYMBOL_list = 44,                      /* list  */
  YYSYMBOL_list0 = 45,                     /* list0  */
  YYSYMBOL_list1 = 46,                     /* list1  */
  YYSYMBOL_newline_list = 47,              /* newline_list  */
  YYSYMBOL_command = 48,                   /* command  */
  YYSYMBOL_shell_command = 49,             /* shell_command  */
  YYSYMBOL_if_statement = 50,              /* if_statement  */
  YYSYMBOL_elif_clause = 51,               /* elif_clause  */
  YYSYMBOL_elsif_clause = 52,              /* elsif_clause  */
  YYSYMBOL_unless_statement = 53,          /* unless_statement  */
  YYSYMBOL_while_statement = 54,           /* while_statement  */
  YYSYMBOL_until_statement = 55,           /* until_statement  */
  YYSYMBOL_for_statement = 56,             /* for_statement  */
  YYSYMBOL_simple_command = 57,            /* simple_command  */
  YYSYMBOL_simple_command_element = 58,    /* simple_command_element  */
  YYSYMBOL_redirect_list = 59,             /* redirect_list  */
  YYSYMBOL_word_list = 60,                 /* word_list  */
  YYSYMBOL_list_terminater = 61,           /* list_terminater  */
  YYSYMBOL_redirect = 62                   /* redirect  */
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
#define YYFINAL  53
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   241

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  37
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  26
/* YYNRULES -- Number of rules.  */
#define YYNRULES  111
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  198

/* YYMAXUTOK -- Last valid token kind.  */
#define YYMAXUTOK   285


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
      31,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,    32,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,    33,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,    35,    34,    36,     2,     2,     2,     2,
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
      25,    26,    27,    28,    29,    30
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
     208,   209
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
  "OR_OR", "OR_AND", "GT", "GT_GT", "AND_GT", "GT_AND", "LT", "LT_AND",
  "LT_GT", "'\\n'", "'&'", "';'", "'|'", "'{'", "'}'", "$accept",
  "inputunit", "terminator", "simple_list", "connector", "pipeline",
  "compound_list", "list", "list0", "list1", "newline_list", "command",
  "shell_command", "if_statement", "elif_clause", "elsif_clause",
  "unless_statement", "while_statement", "until_statement",
  "for_statement", "simple_command", "simple_command_element",
  "redirect_list", "word_list", "list_terminater", "redirect", YY_NULLPTR
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
     285,    10,    38,    59,   124,   123,   125
};
#endif

#define YYPACT_NINF (-157)

#define yypact_value_is_default(Yyn) \
  ((Yyn) == YYPACT_NINF)

#define YYTABLE_NINF (-1)

#define yytable_value_is_error(Yyn) \
  0

  /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static const yytype_int16 yypact[] =
{
      29,  -157,  -157,    84,  -157,  -157,  -157,  -157,     7,    72,
      74,    80,    15,    82,    37,   107,   118,     9,    -8,    -6,
    -157,    77,  -157,  -157,  -157,  -157,  -157,    70,  -157,  -157,
     141,   152,    46,   167,   130,   186,   120,  -157,   176,     3,
      36,   127,   -12,  -157,  -157,  -157,  -157,   125,  -157,  -157,
     187,  -157,  -157,  -157,  -157,  -157,  -157,  -157,  -157,   205,
     205,   205,   205,    77,  -157,  -157,  -157,  -157,   188,  -157,
    -157,   191,  -157,  -157,  -157,  -157,  -157,  -157,   197,  -157,
      -6,  -157,   166,  -157,  -157,  -157,  -157,  -157,  -157,  -157,
     208,  -157,    -1,  -157,  -157,   176,   176,    -6,    -6,  -157,
    -157,  -157,  -157,  -157,   204,   209,    59,  -157,  -157,  -157,
     205,   205,   205,    13,   212,    47,   213,  -157,     4,    30,
    -157,  -157,    -6,    -6,  -157,  -157,  -157,  -157,   112,   146,
    -157,  -157,  -157,   176,   176,    -6,   190,    -6,   190,    -6,
     190,  -157,  -157,  -157,  -157,  -157,  -157,  -157,  -157,  -157,
    -157,  -157,  -157,  -157,    49,   182,   169,   219,  -157,  -157,
    -157,  -157,   140,  -157,    -6,    -6,   215,   223,   138,    63,
     201,  -157,  -157,  -157,  -157,  -157,  -157,  -157,  -157,  -157,
    -157,  -157,  -157,  -157,  -157,  -157,  -157,   165,  -157,   104,
     202,  -157,  -157,  -157,  -157,  -157,  -157,  -157
};

  /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static const yytype_int8 yydefact[] =
{
       0,     3,    81,     0,    29,    29,    29,    29,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     6,    13,
      16,    32,    34,    35,    36,    37,    38,    31,    79,    82,
       0,     0,     0,     0,     0,     0,     0,    17,     0,     0,
       0,     0,    29,    98,   108,   106,   107,   101,   100,    90,
      93,    92,   110,     1,     5,     4,     2,    29,    29,     7,
       8,     0,     0,    33,    83,    80,    99,   109,   104,   103,
      91,    96,    95,   111,    29,    29,    29,    39,     0,    30,
      28,    19,    18,    29,    29,    60,    64,    29,    67,    29,
       0,    29,     0,   102,    94,     0,     0,    11,    12,    15,
      14,    84,   105,    97,     0,     0,    54,    41,    29,    29,
      29,    29,    29,     0,     0,     0,     0,    85,     0,     0,
      29,    29,     9,    10,    29,    29,    42,    43,     0,     0,
      40,    29,    29,     0,     0,    27,    20,    25,    21,    26,
      22,    29,    62,    61,    66,    65,    68,    89,    86,    87,
      88,    29,    29,    29,     0,     0,     0,     0,    46,    47,
      48,    49,    56,    55,    23,    24,     0,     0,     0,     0,
       0,    70,    69,    71,    44,    45,    29,    29,    58,    59,
      63,    75,    29,    29,    73,    72,    74,    50,    57,     0,
       0,    29,    52,    53,    77,    76,    78,    51
};

  /* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -157,  -157,  -157,  -157,  -157,    27,    -5,  -157,  -157,  -157,
     -19,   123,  -157,  -157,  -156,   -99,  -157,  -157,  -157,  -157,
    -157,   214,  -157,  -157,  -157,   -18
};

  /* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int16 yydefgoto[] =
{
      -1,    16,    56,    17,    18,    80,    36,    37,    81,    82,
      38,    20,    21,    22,   128,    78,    23,    24,    25,    26,
      27,    28,    63,   118,   151,    29
};

  /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static const yytype_uint8 yytable[] =
{
      39,    40,    41,    64,   147,   129,   178,   148,    90,    54,
      42,    83,    84,    57,    58,   120,    85,    61,    46,    47,
      48,    91,   141,    92,    59,    60,   142,    19,    62,     1,
      79,   192,     2,     3,   121,   149,     4,   150,    95,    96,
      55,    50,    51,     5,     6,   101,   152,     7,     8,    86,
      68,    69,    87,     9,    10,    11,    12,    13,    14,    15,
     144,    79,   171,   179,   145,   153,   172,   131,   132,   104,
     105,   106,   119,     2,     3,    43,   184,    44,   113,   114,
     185,     3,   115,    45,   116,    49,    97,    98,   193,   133,
     134,   136,   138,   140,     9,    10,    11,    12,    13,    14,
      15,     9,    10,    11,    12,    13,    14,    15,    30,    31,
      52,    32,    33,    34,    35,   154,   155,   194,    53,   156,
     157,   195,   122,   123,   158,   159,   162,   163,    74,    75,
      93,    76,   168,    77,    71,    72,   166,   135,   137,   139,
      88,     2,     3,    89,    66,     4,   167,   169,   170,   177,
     125,    76,     5,     6,   182,    67,     7,     8,   160,   161,
     164,   165,     9,    10,    11,    12,    13,    14,    15,    79,
      70,   187,   188,   183,   191,   125,    76,   189,   190,     2,
       3,   174,   175,     4,    99,   100,   197,   108,   109,    73,
       5,     6,    94,   102,     7,     8,   103,   110,   111,   112,
       9,    10,    11,    12,    13,    14,    15,    79,     2,     3,
     107,   117,     4,   124,   125,    76,   126,   127,   173,     5,
       6,    79,   130,     7,     8,   143,   146,   176,   180,     9,
      10,    11,    12,    13,    14,    15,   181,   186,   196,     0,
       0,    65
};

static const yytype_int16 yycheck[] =
{
       5,     6,     7,    21,     0,   104,   162,     3,    20,     0,
       3,     8,     9,    21,    22,    16,    13,    23,     3,     4,
       5,    33,     9,    42,    32,    33,    13,     0,    34,     0,
      31,   187,     3,     4,    35,    31,     7,    33,    57,    58,
      31,     4,     5,    14,    15,    63,    16,    18,    19,    13,
       4,     5,    16,    24,    25,    26,    27,    28,    29,    30,
      13,    31,    13,   162,    17,    35,    17,     8,     9,    74,
      75,    76,    91,     3,     4,     3,    13,     3,    83,    84,
      17,     4,    87,     3,    89,     3,    59,    60,   187,   108,
     109,   110,   111,   112,    24,    25,    26,    27,    28,    29,
      30,    24,    25,    26,    27,    28,    29,    30,    24,    25,
       3,    27,    28,    29,    30,   120,   121,    13,     0,   124,
     125,    17,    95,    96,    12,    13,   131,   132,     8,     9,
       5,    11,   151,    13,     4,     5,   141,   110,   111,   112,
      13,     3,     4,    16,     3,     7,   151,   152,   153,     9,
      10,    11,    14,    15,    16,     3,    18,    19,    12,    13,
     133,   134,    24,    25,    26,    27,    28,    29,    30,    31,
       3,   176,   177,    35,     9,    10,    11,   182,   183,     3,
       4,    12,    13,     7,    61,    62,   191,    21,    22,     3,
      14,    15,     5,     5,    18,    19,     5,    31,    32,    33,
      24,    25,    26,    27,    28,    29,    30,    31,     3,     4,
      13,     3,     7,     9,    10,    11,    12,    13,    36,    14,
      15,    31,    13,    18,    19,    13,    13,     8,    13,    24,
      25,    26,    27,    28,    29,    30,    13,    36,    36,    -1,
      -1,    27
};

  /* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static const yytype_int8 yystos[] =
{
       0,     0,     3,     4,     7,    14,    15,    18,    19,    24,
      25,    26,    27,    28,    29,    30,    38,    40,    41,    42,
      48,    49,    50,    53,    54,    55,    56,    57,    58,    62,
      24,    25,    27,    28,    29,    30,    43,    44,    47,    43,
      43,    43,     3,     3,     3,     3,     3,     4,     5,     3,
       4,     5,     3,     0,     0,    31,    39,    21,    22,    32,
      33,    23,    34,    59,    62,    58,     3,     3,     4,     5,
       3,     4,     5,     3,     8,     9,    11,    13,    52,    31,
      42,    45,    46,     8,     9,    13,    13,    16,    13,    16,
      20,    33,    47,     5,     5,    47,    47,    42,    42,    48,
      48,    62,     5,     5,    43,    43,    43,    13,    21,    22,
      31,    32,    33,    43,    43,    43,    43,     3,    60,    47,
      16,    35,    42,    42,     9,    10,    12,    13,    51,    52,
      13,     8,     9,    47,    47,    42,    47,    42,    47,    42,
      47,     9,    13,    13,    13,    17,    13,     0,     3,    31,
      33,    61,    16,    35,    43,    43,    43,    43,    12,    13,
      12,    13,    43,    43,    42,    42,    43,    43,    47,    43,
      43,    13,    17,    36,    12,    13,     8,     9,    51,    52,
      13,    13,    16,    35,    13,    17,    36,    43,    43,    43,
      43,     9,    51,    52,    13,    17,    36,    43
};

  /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_int8 yyr1[] =
{
       0,    37,    38,    38,    39,    39,    40,    40,    40,    41,
      41,    41,    41,    41,    42,    42,    42,    43,    43,    44,
      45,    45,    45,    46,    46,    46,    46,    46,    46,    47,
      47,    48,    48,    48,    49,    49,    49,    49,    49,    50,
      50,    50,    50,    50,    50,    50,    50,    50,    50,    50,
      51,    51,    51,    51,    52,    52,    52,    52,    52,    52,
      53,    53,    53,    53,    54,    54,    54,    55,    55,    56,
      56,    56,    56,    56,    56,    56,    56,    56,    56,    57,
      57,    58,    58,    59,    59,    60,    60,    61,    61,    61,
      62,    62,    62,    62,    62,    62,    62,    62,    62,    62,
      62,    62,    62,    62,    62,    62,    62,    62,    62,    62,
      62,    62
};

  /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static const yytype_int8 yyr2[] =
{
       0,     2,     2,     1,     1,     1,     1,     2,     2,     4,
       4,     3,     3,     1,     3,     3,     1,     1,     2,     2,
       3,     3,     3,     4,     4,     3,     3,     3,     1,     0,
       2,     1,     1,     2,     1,     1,     1,     1,     1,     3,
       5,     4,     5,     5,     7,     7,     6,     6,     6,     6,
       4,     6,     5,     5,     2,     4,     4,     6,     5,     5,
       3,     5,     5,     7,     3,     5,     5,     3,     5,     6,
       6,     6,     7,     7,     7,     7,     9,     9,     9,     1,
       2,     1,     1,     1,     2,     1,     2,     1,     1,     1,
       2,     3,     2,     2,     3,     3,     3,     4,     2,     3,
       2,     2,     3,     3,     3,     4,     2,     2,     2,     3,
       2,     3
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
#line 1368 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 3: /* inputunit: $end  */
#line 58 "mrbgems/mruby-reddish-parser/core/parser.y"
                         { p->result = NIL;YYACCEPT;}
#line 1374 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 7: /* simple_list: connector '&'  */
#line 64 "mrbgems/mruby-reddish-parser/core/parser.y"
                { yyval = ASYNC(p, yyvsp[-1]); }
#line 1380 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 9: /* connector: connector AND_AND newline_list pipeline  */
#line 68 "mrbgems/mruby-reddish-parser/core/parser.y"
                                          { yyval = CONNECTOR(p, "and", yyvsp[-3], yyvsp[0]); }
#line 1386 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 10: /* connector: connector OR_OR newline_list pipeline  */
#line 69 "mrbgems/mruby-reddish-parser/core/parser.y"
                                          { yyval = CONNECTOR(p, "or",  yyvsp[-3], yyvsp[0]); }
#line 1392 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 11: /* connector: connector '&' pipeline  */
#line 70 "mrbgems/mruby-reddish-parser/core/parser.y"
                                          { yyval = CONNECTOR(p, "async", yyvsp[-2], yyvsp[0]); }
#line 1398 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 12: /* connector: connector ';' pipeline  */
#line 71 "mrbgems/mruby-reddish-parser/core/parser.y"
                                          { yyval = CONNECTOR(p, "semicolon", yyvsp[-2], yyvsp[0]); }
#line 1404 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 14: /* pipeline: pipeline '|' command  */
#line 75 "mrbgems/mruby-reddish-parser/core/parser.y"
                          { yyval = PIPELINE(p, yyvsp[-2], yyvsp[0], MRB_FALSE); }
#line 1410 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 15: /* pipeline: pipeline OR_AND command  */
#line 76 "mrbgems/mruby-reddish-parser/core/parser.y"
                          { yyval = PIPELINE(p, yyvsp[-2], yyvsp[0], MRB_TRUE); }
#line 1416 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 18: /* compound_list: newline_list list1  */
#line 81 "mrbgems/mruby-reddish-parser/core/parser.y"
                     { yyval = yyvsp[0]; }
#line 1422 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 19: /* list: newline_list list0  */
#line 84 "mrbgems/mruby-reddish-parser/core/parser.y"
                     { yyval = yyvsp[0]; }
#line 1428 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 21: /* list0: list1 '&' newline_list  */
#line 88 "mrbgems/mruby-reddish-parser/core/parser.y"
                          { yyval = ASYNC(p, yyvsp[-2]); }
#line 1434 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 23: /* list1: list1 AND_AND newline_list pipeline  */
#line 92 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "and", yyvsp[-3], yyvsp[0]); }
#line 1440 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 24: /* list1: list1 OR_OR newline_list pipeline  */
#line 93 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "or",  yyvsp[-3], yyvsp[0]); }
#line 1446 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 25: /* list1: list1 '&' pipeline  */
#line 94 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "async", yyvsp[-2], yyvsp[0]); }
#line 1452 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 26: /* list1: list1 ';' pipeline  */
#line 95 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "semicolon", yyvsp[-2], yyvsp[0]); }
#line 1458 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 27: /* list1: list1 '\n' pipeline  */
#line 96 "mrbgems/mruby-reddish-parser/core/parser.y"
                                      { yyval = CONNECTOR(p, "semicolon", yyvsp[-2], yyvsp[0]); }
#line 1464 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 31: /* command: simple_command  */
#line 104 "mrbgems/mruby-reddish-parser/core/parser.y"
                 { yyval = COMMAND(p, yyvsp[0]); }
#line 1470 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 33: /* command: shell_command redirect_list  */
#line 106 "mrbgems/mruby-reddish-parser/core/parser.y"
                              { APPEND_REDIRECT(p, yyvsp[-1], yyvsp[0]); yyval = yyvsp[-1]; }
#line 1476 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 39: /* if_statement: IF compound_list END  */
#line 116 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-1], 0, NIL); }
#line 1482 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 40: /* if_statement: IF compound_list ELSE compound_list END  */
#line 117 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 2, NIL, yyvsp[-1]); }
#line 1488 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 41: /* if_statement: IF compound_list elsif_clause END  */
#line 118 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-2], 2, NIL, yyvsp[-1]); }
#line 1494 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 42: /* if_statement: IF compound_list THEN compound_list FI  */
#line 119 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 1, yyvsp[-1]); }
#line 1500 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 43: /* if_statement: IF compound_list THEN compound_list END  */
#line 120 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 1, yyvsp[-1]); }
#line 1506 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 44: /* if_statement: IF compound_list THEN compound_list ELSE compound_list FI  */
#line 121 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-5], 2, yyvsp[-3], yyvsp[-1]); }
#line 1512 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 45: /* if_statement: IF compound_list THEN compound_list ELSE compound_list END  */
#line 122 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-5], 2, yyvsp[-3], yyvsp[-1]); }
#line 1518 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 46: /* if_statement: IF compound_list THEN compound_list elif_clause FI  */
#line 123 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[-1]); }
#line 1524 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 47: /* if_statement: IF compound_list THEN compound_list elif_clause END  */
#line 124 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[-1]); }
#line 1530 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 48: /* if_statement: IF compound_list THEN compound_list elsif_clause FI  */
#line 125 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[-1]); }
#line 1536 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 49: /* if_statement: IF compound_list THEN compound_list elsif_clause END  */
#line 126 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[-1]); }
#line 1542 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 50: /* elif_clause: ELIF compound_list THEN compound_list  */
#line 129 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                           { yyval = IF_STMT(p, yyvsp[-2], 1, yyvsp[0]); }
#line 1548 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 51: /* elif_clause: ELIF compound_list THEN compound_list ELSE compound_list  */
#line 130 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                           { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[0]); }
#line 1554 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 52: /* elif_clause: ELIF compound_list THEN compound_list elif_clause  */
#line 131 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                           { yyval = IF_STMT(p, yyvsp[-3], 2, yyvsp[-1], yyvsp[0]); }
#line 1560 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 53: /* elif_clause: ELIF compound_list THEN compound_list elsif_clause  */
#line 132 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                           { yyval = IF_STMT(p, yyvsp[-3], 2, yyvsp[-1], yyvsp[0]); }
#line 1566 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 54: /* elsif_clause: ELSIF compound_list  */
#line 135 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[0], 0, NIL); }
#line 1572 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 55: /* elsif_clause: ELSIF compound_list ELSE compound_list  */
#line 136 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-2], 2, NIL, yyvsp[0]); }
#line 1578 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 56: /* elsif_clause: ELSIF compound_list THEN compound_list  */
#line 137 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-2], 1, yyvsp[0]); }
#line 1584 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 57: /* elsif_clause: ELSIF compound_list THEN compound_list ELSE compound_list  */
#line 138 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-4], 2, yyvsp[-2], yyvsp[0]); }
#line 1590 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 58: /* elsif_clause: ELSIF compound_list THEN compound_list elif_clause  */
#line 139 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 2, yyvsp[-1], yyvsp[0]); }
#line 1596 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 59: /* elsif_clause: ELSIF compound_list THEN compound_list elsif_clause  */
#line 140 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                            { yyval = IF_STMT(p, yyvsp[-3], 2, yyvsp[-1], yyvsp[0]); }
#line 1602 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 60: /* unless_statement: UNLESS compound_list END  */
#line 143 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                 { yyval = UNLESS_STMT(p, yyvsp[-1], 0, NIL); }
#line 1608 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 61: /* unless_statement: UNLESS compound_list ELSE compound_list END  */
#line 144 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                 { yyval = UNLESS_STMT(p, yyvsp[-3], 2, NIL, yyvsp[-1]); }
#line 1614 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 62: /* unless_statement: UNLESS compound_list THEN compound_list END  */
#line 145 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                 { yyval = UNLESS_STMT(p, yyvsp[-3], 1, yyvsp[-1]); }
#line 1620 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 63: /* unless_statement: UNLESS compound_list THEN compound_list ELSE compound_list END  */
#line 146 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                 { yyval = UNLESS_STMT(p, yyvsp[-5], 2, yyvsp[-3], yyvsp[-1]); }
#line 1626 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 64: /* while_statement: WHILE compound_list END  */
#line 149 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = WHILE_STMT(p, yyvsp[-1], NIL); }
#line 1632 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 65: /* while_statement: WHILE compound_list DO compound_list DONE  */
#line 150 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = WHILE_STMT(p, yyvsp[-3], yyvsp[-1]); }
#line 1638 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 66: /* while_statement: WHILE compound_list DO compound_list END  */
#line 151 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = WHILE_STMT(p, yyvsp[-3], yyvsp[-1]); }
#line 1644 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 67: /* until_statement: UNTIL compound_list END  */
#line 154 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = UNTIL_STMT(p, yyvsp[-1], NIL); }
#line 1650 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 68: /* until_statement: UNTIL compound_list DO compound_list END  */
#line 155 "mrbgems/mruby-reddish-parser/core/parser.y"
                                            { yyval = UNTIL_STMT(p, yyvsp[-3], yyvsp[-1]); }
#line 1656 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 69: /* for_statement: FOR WORD newline_list DO compound_list DONE  */
#line 158 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-4], NIL, yyvsp[-1]); }
#line 1662 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 70: /* for_statement: FOR WORD newline_list DO compound_list END  */
#line 159 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-4], NIL, yyvsp[-1]); }
#line 1668 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 71: /* for_statement: FOR WORD newline_list '{' compound_list '}'  */
#line 160 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-4], NIL, yyvsp[-1]); }
#line 1674 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 72: /* for_statement: FOR WORD ';' newline_list DO compound_list DONE  */
#line 161 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-5], NIL, yyvsp[-1]); }
#line 1680 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 73: /* for_statement: FOR WORD ';' newline_list DO compound_list END  */
#line 162 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-5], NIL, yyvsp[-1]); }
#line 1686 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 74: /* for_statement: FOR WORD ';' newline_list '{' compound_list '}'  */
#line 163 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                          { yyval = FOR_STMT(p, yyvsp[-5], NIL, yyvsp[-1]); }
#line 1692 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 75: /* for_statement: FOR WORD IN word_list list_terminater compound_list END  */
#line 164 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                              { yyval = FOR_STMT(p, yyvsp[-5], yyvsp[-3], yyvsp[-1]); }
#line 1698 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 76: /* for_statement: FOR WORD IN word_list list_terminater newline_list DO compound_list DONE  */
#line 165 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                           { yyval = FOR_STMT(p, yyvsp[-7], yyvsp[-5], yyvsp[-1]); }
#line 1704 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 77: /* for_statement: FOR WORD IN word_list list_terminater newline_list DO compound_list END  */
#line 166 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                           { yyval = FOR_STMT(p, yyvsp[-7], yyvsp[-5], yyvsp[-1]); }
#line 1710 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 78: /* for_statement: FOR WORD IN word_list list_terminater newline_list '{' compound_list '}'  */
#line 167 "mrbgems/mruby-reddish-parser/core/parser.y"
                                                                           { yyval = FOR_STMT(p, yyvsp[-7], yyvsp[-5], yyvsp[-1]); }
#line 1716 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 79: /* simple_command: simple_command_element  */
#line 170 "mrbgems/mruby-reddish-parser/core/parser.y"
                         { yyval = mrb_ary_new_from_values(p->state, 1, &yyvsp[0]); }
#line 1722 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 80: /* simple_command: simple_command simple_command_element  */
#line 171 "mrbgems/mruby-reddish-parser/core/parser.y"
                                        { mrb_ary_push(p->state, yyvsp[-1], yyvsp[0]); yyval = yyvsp[-1]; }
#line 1728 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 81: /* simple_command_element: WORD  */
#line 174 "mrbgems/mruby-reddish-parser/core/parser.y"
       { yyval = WORD(p, yyvsp[0]); }
#line 1734 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 84: /* redirect_list: redirect_list redirect  */
#line 179 "mrbgems/mruby-reddish-parser/core/parser.y"
                         { mrb_ary_concat(p->state, yyvsp[-1], yyvsp[0]); yyval = yyvsp[-1]; }
#line 1740 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 85: /* word_list: WORD  */
#line 182 "mrbgems/mruby-reddish-parser/core/parser.y"
       { yyval = mrb_ary_new_from_values(p->state, 1, &yyvsp[0]); }
#line 1746 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 86: /* word_list: word_list WORD  */
#line 183 "mrbgems/mruby-reddish-parser/core/parser.y"
                 { mrb_ary_push(p->state, yyvsp[-1], yyvsp[0]); yyval = yyvsp[-1]; }
#line 1752 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 90: /* redirect: LT WORD  */
#line 188 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "read",     2, FIXNUM(0), yyvsp[0]); }
#line 1758 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 91: /* redirect: NUMBER LT WORD  */
#line 189 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "read",     2, yyvsp[-2], yyvsp[0]); }
#line 1764 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 92: /* redirect: LT_AND MINUS  */
#line 190 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "close",    1, FIXNUM(0)); }
#line 1770 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 93: /* redirect: LT_AND NUMBER  */
#line 191 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyread", 2, FIXNUM(0), yyvsp[0]); }
#line 1776 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 94: /* redirect: LT_AND NUMBER MINUS  */
#line 192 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyreadclose", 2, FIXNUM(0), yyvsp[-1]); }
#line 1782 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 95: /* redirect: NUMBER LT_AND MINUS  */
#line 193 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "close",    1, yyvsp[-2]); }
#line 1788 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 96: /* redirect: NUMBER LT_AND NUMBER  */
#line 194 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyread", 2, yyvsp[-2], yyvsp[0]); }
#line 1794 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 97: /* redirect: NUMBER LT_AND NUMBER MINUS  */
#line 195 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copyreadclose", 2, yyvsp[-3], yyvsp[-1]); }
#line 1800 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 98: /* redirect: GT WORD  */
#line 196 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "write",     2, FIXNUM(1), yyvsp[0]); }
#line 1806 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 99: /* redirect: NUMBER GT WORD  */
#line 197 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "write",     2, yyvsp[-2], yyvsp[0]); }
#line 1812 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 100: /* redirect: GT_AND MINUS  */
#line 198 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "close",     1, FIXNUM(1)); }
#line 1818 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 101: /* redirect: GT_AND NUMBER  */
#line 199 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copywrite", 2, FIXNUM(1), yyvsp[0]); }
#line 1824 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 102: /* redirect: GT_AND NUMBER MINUS  */
#line 200 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copywriteclose", 2, FIXNUM(1), yyvsp[-1]); }
#line 1830 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 103: /* redirect: NUMBER GT_AND MINUS  */
#line 201 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "close",     1, yyvsp[-2]); }
#line 1836 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 104: /* redirect: NUMBER GT_AND NUMBER  */
#line 202 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copywrite", 2, yyvsp[-2], yyvsp[0]); }
#line 1842 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 105: /* redirect: NUMBER GT_AND NUMBER MINUS  */
#line 203 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copywriteclose", 2, yyvsp[-3], yyvsp[-1]); }
#line 1848 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 106: /* redirect: AND_GT WORD  */
#line 204 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copystdoutstderr", 3, FIXNUM(1), FIXNUM(2), yyvsp[0]); }
#line 1854 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 107: /* redirect: GT_AND WORD  */
#line 205 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "copystdoutstderr", 3, FIXNUM(1), FIXNUM(2), yyvsp[0]); }
#line 1860 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 108: /* redirect: GT_GT WORD  */
#line 206 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "append", 2, FIXNUM(1), yyvsp[0]); }
#line 1866 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 109: /* redirect: NUMBER GT_GT WORD  */
#line 207 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "append", 2, yyvsp[-2], yyvsp[0]); }
#line 1872 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 110: /* redirect: LT_GT WORD  */
#line 208 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "readwrite", 2, FIXNUM(0), yyvsp[0]); }
#line 1878 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;

  case 111: /* redirect: NUMBER LT_GT WORD  */
#line 209 "mrbgems/mruby-reddish-parser/core/parser.y"
                                       { yyval = REDIRECT(p, "readwrite", 2, yyvsp[-2], yyvsp[0]); }
#line 1884 "mrbgems/mruby-reddish-parser/src/parser.c"
    break;


#line 1888 "mrbgems/mruby-reddish-parser/src/parser.c"

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

#line 211 "mrbgems/mruby-reddish-parser/core/parser.y"

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
    {">",  GT},
    {">&", GT_AND},
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
