# Model report for file:///tmp/top-repos-quality-repos-aztcvc51/informatica-code-a-thon-2018.git HEAD 6399ba3d4d73d2efc42ab179b04b97028fcc126e

### Dump

```json
{'created_at': '2021-08-20 18:56:40',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '27.7 kB',
 'tags': [],
 'uuid': '91340f24-40ed-4297-8c11-35b6d22dc16e',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-aztcvc51/informatica-code-a-thon-2018.git 6399ba3d4d73d2efc42ab179b04b97028fcc126e

# javascript
66 rules, avg.len. 9.8
## train
PPCR: 0.804448
### report
macro
{'f1-score': 0.28088919753321295,
 'precision': 0.320835028984126,
 'recall': 0.2704462711780921,
 'support': 120964}
micro
{'f1-score': 0.9606081148110183,
 'precision': 0.9606081148110182,
 'recall': 0.9606081148110182,
 'support': 120964}
weighted
{'f1-score': 0.9544258370392755,
 'precision': 0.9518336559114313,
 'recall': 0.9606081148110182,
 'support': 120964}
### report_full
macro
{'f1-score': 0.20072667258120244,
 'precision': 0.320835028984126,
 'recall': 0.17332882795733923,
 'support': 150369}
micro
{'f1-score': 0.8565047377208079,
 'precision': 0.9606081148110182,
 'recall': 0.7727590128284421,
 'support': 150369}
weighted
{'f1-score': 0.8169047456160642,
 'precision': 0.9182396801755359,
 'recall': 0.7727590128284421,
 'support': 150369}
## test
PPCR: 0.776623
### report
macro
{'f1-score': 0.27253873379160454,
 'precision': 0.3080100998799167,
 'recall': 0.27376810520791334,
 'support': 10910}
micro
{'f1-score': 0.9547204399633364,
 'precision': 0.9547204399633364,
 'recall': 0.9547204399633364,
 'support': 10910}
weighted
{'f1-score': 0.945458826186908,
 'precision': 0.9452300660609549,
 'recall': 0.9547204399633364,
 'support': 10910}
### report_full
macro
{'f1-score': 0.20404449023226084,
 'precision': 0.3080100998799167,
 'recall': 0.18247115760936636,
 'support': 14048}
micro
{'f1-score': 0.8346822662072281,
 'precision': 0.9547204399633364,
 'recall': 0.7414578587699316,
 'support': 14048}
weighted
{'f1-score': 0.7787630444172833,
 'precision': 0.8890444573858455,
 'recall': 0.7414578587699316,
 'support': 14048}
```

## javascript
### Summary
42 rules, avg.len. 9.5

| | |
|-|-|
|Min support|92|
|Max support|39575|
|Min confidence|0.921875|
|Max confidence|0.9994925856590271|

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
| 1 | `  -1.label in {<space>}<br>	∧ +3.reserved = {<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.956. Support: 400.` |
| 2 | `  -1.label not in {<space>}<br>	∧ -1.roles in {STRING}<br>	∧ ^1.roles in {OPERATOR, RELATIONAL}<br>⇒ y = "<br>Confidence: 0.922. Support: 96.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 714.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 332.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 264.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≤ 3<br>	∧ -3.roles not in {STRING}<br>	∧ +1.reserved = (<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.981. Support: 232.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≤ 2<br>	∧ -2.internal_type = Identifier<br>	∧ -3.roles not in {STRING}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 123.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {+}<br>	∧ -3.reserved = =<br>	∧ -3.roles not in {STRING}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 92.` |
| 9 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ -2.reserved not in {+}<br>	∧ -3.reserved not in {=}<br>	∧ -3.roles not in {STRING}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.956. Support: 11183.` |
| 10 | `  -1.roles in {STRING}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {MAP}<br>⇒ y = '<br>Confidence: 0.949. Support: 841.` |
| 11 | `  -1.roles in {STRING}<br>	∧ ^1.roles in {ASSIGNMENT} and not in {OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = "<br>Confidence: 0.945. Support: 100.` |
| 12 | `  -1.roles in {STRING}<br>	∧ +2.reserved = {<br>	∧ ^1.roles not in {ASSIGNMENT, OPERATOR}<br>	∧ ^2.roles not in {MAP}<br>⇒ y = '<br>Confidence: 0.963. Support: 468.` |
| 13 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.924. Support: 648.` |
| 14 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.label not in {'}<br>	∧ -4.roles in {KEY}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.996. Support: 142.` |
| 15 | `  -1.reserved not in {,, :}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -2.length ≤ 6<br>	∧ -4.label not in {'}<br>	∧ -5.diff_offset ≤ 14<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.927. Support: 225.` |
| 16 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 3666.` |
| 17 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.937. Support: 1045.` |
| 18 | `  -1.reserved = ,<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {}}<br>	∧ -4.diff_offset ≤ 33<br>	∧ -4.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.length ≤ 5<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ␣<br>Confidence: 0.952. Support: 2180.` |
| 19 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 218.` |
| 20 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 500.` |
| 21 | `  -1.reserved not in {(, ,, :, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved = }<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 280.` |
| 22 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ -3.reserved not in {}}<br>	∧ -5.reserved = function<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 109.` |
| 23 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {}}<br>	∧ -4.reserved = function<br>	∧ -5.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 147.` |
| 24 | `  -1.reserved = )<br>	∧ -1.roles not in {STRING}<br>	∧ -3.diff_col ≤ 4<br>	∧ -3.reserved not in {}}<br>	∧ -5.diff_col ≤ 8<br>	∧ -5.roles not in {ARGUMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +4.roles not in {BINARY}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 92.` |
| 25 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1436.` |
| 26 | `  -1.reserved = :<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {'}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 1333.` |
| 27 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 470.` |
| 28 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = ?<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 151.` |
| 29 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, var}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 117.` |
| 30 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, :, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), {}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 103.` |
| 31 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 1471.` |
| 32 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = return<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 198.` |
| 33 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = else<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 140.` |
| 34 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :, ;, var, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<newline>}<br>	∧ -2.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1272.` |
| 35 | `  -1.internal_type = Identifier<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :, ;, var}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 8869.` |
| 36 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {,, :, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.roles not in {STRING}<br>	∧ -4.diff_line = 0<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +3.roles in {FUNCTION}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 130.` |
| 37 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(}<br>	∧ +1.roles not in {ASSIGNMENT}<br>	∧ +1.length ≥ 5<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 137.` |
| 38 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {,, :, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {ASSIGNMENT}<br>	∧ +1.length ≤ 4<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 1908.` |
| 39 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {,, :, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {ASSIGNMENT}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 513.` |
| 40 | `  -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {,, :, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.diff_col ≥ 8<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {ASSIGNMENT}<br>	∧ +3.roles in {BINARY}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 115.` |
| 41 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {,, :, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {ASSIGNMENT}<br>	∧ +3.roles in {BINARY}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2101.` |
| 42 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>, <tab>}<br>	∧ -1.reserved not in {,, :, ;, var, {, }}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.reserved not in {}}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {ASSIGNMENT}<br>	∧ +3.roles not in {BINARY}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 39575.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.523809523809524, "max_conf": 0.9994925856590271, "max_support": 39575, "min_conf": 0.921875, "min_support": 92, "num_rules": 42}}
```
</details>
