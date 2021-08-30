# Model report for file:///tmp/top-repos-quality-repos-my3guemq/chakra-ui-vue.git HEAD 8984d3cb11cb5e494fb8abd01046c6f2b77d6118

### Dump

```json
{'created_at': '2021-08-29 14:39:54',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '17.8 kB',
 'tags': [],
 'uuid': 'cc8459b8-122b-45d7-98ae-9e4518f0875c',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-my3guemq/chakra-ui-vue.git 8984d3cb11cb5e494fb8abd01046c6f2b77d6118

# javascript
46 rules, avg.len. 10.1
## train
PPCR: 0.943591
### report
macro
{'f1-score': 0.8094285778448025,
 'precision': 0.8377681524137773,
 'recall': 0.7904328816572177,
 'support': 71896}
micro
{'f1-score': 0.9652553688661399,
 'precision': 0.9652553688661399,
 'recall': 0.9652553688661399,
 'support': 71896}
weighted
{'f1-score': 0.9641394319598475,
 'precision': 0.9638400702581148,
 'recall': 0.9652553688661399,
 'support': 71896}
### report_full
macro
{'f1-score': 0.7487762474865632,
 'precision': 0.8377681524137773,
 'recall': 0.7073240181389422,
 'support': 76194}
micro
{'f1-score': 0.9372408670403133,
 'precision': 0.9652553688661399,
 'recall': 0.9108066251935848,
 'support': 76194}
weighted
{'f1-score': 0.9321654875295076,
 'precision': 0.9629709515123033,
 'recall': 0.9108066251935848,
 'support': 76194}
## test
PPCR: 0.948699
### report
macro
{'f1-score': 0.8162357978612338,
 'precision': 0.8395584196442454,
 'recall': 0.7990101874764348,
 'support': 16773}
micro
{'f1-score': 0.9681034996720921,
 'precision': 0.9681034996720921,
 'recall': 0.9681034996720921,
 'support': 16773}
weighted
{'f1-score': 0.9676922664520341,
 'precision': 0.9681011417179047,
 'recall': 0.9681034996720921,
 'support': 16773}
### report_full
macro
{'f1-score': 0.7581428577169991,
 'precision': 0.8395584196442454,
 'recall': 0.717606592794192,
 'support': 17680}
micro
{'f1-score': 0.942617478884277,
 'precision': 0.9681034996720921,
 'recall': 0.9184389140271493,
 'support': 17680}
weighted
{'f1-score': 0.9380286169623963,
 'precision': 0.9677535858645391,
 'recall': 0.9184389140271493,
 'support': 17680}
```

## javascript
### Summary
36 rules, avg.len. 9.4

| | |
|-|-|
|Min support|99|
|Max support|12114|
|Min confidence|0.9225721955299377|
|Max confidence|0.9990875720977783|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.982. Support: 3233.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 423.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 284.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), ,}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1910.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 548.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 136.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +4.length ≥ 1<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 222.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ +2.reserved = )<br>	∧ +4.length ≥ 1<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 131.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +4.length ≥ 1<br>	∧ ^1.internal_type not in {BlockStatement, ConditionalExpression}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 12114.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {KEY}<br>⇒ y = '<br>Confidence: 0.992. Support: 184.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>}<br>	∧ -1.reserved not in {{}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {KEY}<br>⇒ y = ⏎<br>Confidence: 0.963. Support: 3051.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.958. Support: 108.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.label not in {<-space>}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.988. Support: 3829.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.997. Support: 2042.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles in {STRING} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.989. Support: 653.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {KEY, STRING}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2903.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.940. Support: 1972.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 1523.` |
| 19 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type = CommentBlock<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 223.` |
| 20 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentBlock}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = '<br>Confidence: 0.996. Support: 113.` |
| 21 | `  -1.diff_col ≥ 9<br>	∧ -1.internal_type not in {CommentBlock, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type = File<br>⇒ y = ⏎<br>Confidence: 0.969. Support: 244.` |
| 22 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.923. Support: 381.` |
| 23 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = ...<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {KEY}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 294.` |
| 24 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {IDENTIFIER, KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.995. Support: 99.` |
| 25 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -1.reserved not in {(, ,}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {IDENTIFIER, KEY}<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 164.` |
| 26 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ...}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {COMMENT} and not in {KEY}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 162.` |
| 27 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ...}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ -2.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 621.` |
| 28 | `  -1.diff_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ...}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles not in {STRING}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, INCOMPLETE, LITERAL}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 10556.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 5<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY, VALUE}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.982. Support: 1185.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_offset ≤ 2<br>	∧ -3.diff_offset ≤ 5<br>	∧ -3.label in {<space>}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 1308.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 5<br>	∧ -3.label not in {<space>}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 285.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 5<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles in {MAP}<br>	∧ +1.reserved not in {,, {, }}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY, LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 325.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 5<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.reserved not in {,, {, }}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {MODULE}<br>⇒ y = ⏎⏎<br>Confidence: 0.943. Support: 131.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 5<br>	∧ -3.label not in {<space>}<br>	∧ -4.roles not in {MAP}<br>	∧ +1.reserved not in {,, }}<br>	∧ +1.roles in {EXPRESSION} and not in {KEY}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BODY, LITERAL, MODULE}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 173.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≤ 5<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.reserved not in {,, {, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.length ≥ 7<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ⏎⏎<br>Confidence: 0.961. Support: 190.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length = 0<br>	∧ -3.diff_offset ≤ 5<br>	∧ -3.label not in {<space>}<br>	∧ -4.diff_line = 0<br>	∧ +1.reserved not in {,, {, }}<br>	∧ +1.roles not in {EXPRESSION, KEY}<br>	∧ +2.length ≥ 7<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 99.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.36111111111111, "max_conf": 0.9990875720977783, "max_support": 12114, "min_conf": 0.9225721955299377, "min_support": 99, "num_rules": 36}}
```
</details>
