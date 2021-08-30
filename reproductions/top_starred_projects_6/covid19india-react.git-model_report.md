# Model report for file:///tmp/top-repos-quality-repos-dr3uypbr/covid19india-react.git HEAD b9add2bedc564b94aadacc7bb164372115274eb6

### Dump

```json
{'created_at': '2021-08-29 22:04:54',
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
 'size': '22.5 kB',
 'tags': [],
 'uuid': '13a34d18-b66f-4e39-b850-9a995687f738',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-dr3uypbr/covid19india-react.git b9add2bedc564b94aadacc7bb164372115274eb6

# javascript
193 rules, avg.len. 9.1
## train
PPCR: 0.939061
### report
macro
{'f1-score': 0.6666581400286313,
 'precision': 0.7300081448418683,
 'recall': 0.6334510712207397,
 'support': 38432}
micro
{'f1-score': 0.923423189009159,
 'precision': 0.923423189009159,
 'recall': 0.923423189009159,
 'support': 38432}
weighted
{'f1-score': 0.914827702169269,
 'precision': 0.9168236668756229,
 'recall': 0.923423189009159,
 'support': 38432}
### report_full
macro
{'f1-score': 0.6104418695826517,
 'precision': 0.7300081448418683,
 'recall': 0.5551627720402486,
 'support': 40926}
micro
{'f1-score': 0.8944025807101994,
 'precision': 0.923423189009159,
 'recall': 0.8671504666959878,
 'support': 40926}
weighted
{'f1-score': 0.8813721759187981,
 'precision': 0.9147261787401928,
 'recall': 0.8671504666959878,
 'support': 40926}
## test
PPCR: 0.941102
### report
macro
{'f1-score': 0.672669640227856,
 'precision': 0.7304141242025401,
 'recall': 0.6422644732666679,
 'support': 9635}
micro
{'f1-score': 0.924545926310327,
 'precision': 0.924545926310327,
 'recall': 0.924545926310327,
 'support': 9635}
weighted
{'f1-score': 0.916586674311453,
 'precision': 0.9187014025824966,
 'recall': 0.924545926310327,
 'support': 9635}
### report_full
macro
{'f1-score': 0.6206509904339005,
 'precision': 0.7304141242025401,
 'recall': 0.565962808438992,
 'support': 10238}
micro
{'f1-score': 0.8964927288280582,
 'precision': 0.924545926310327,
 'recall': 0.8700918148075796,
 'support': 10238}
weighted
{'f1-score': 0.884869581859537,
 'precision': 0.9161873456295684,
 'recall': 0.8700918148075796,
 'support': 10238}
```

## javascript
### Summary
110 rules, avg.len. 9.0

| | |
|-|-|
|Min support|134|
|Max support|14555|
|Min confidence|0.9219188690185547|
|Max confidence|0.9995915293693542|

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
                     'base_model_name': 'sklearn.ensemble.RandomForestClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 181,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.997. Support: 155.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -3.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 1.000. Support: 1118.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.950. Support: 392.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.986. Support: 180.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.996. Support: 142.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {MAP}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 143.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 727.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.974. Support: 957.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 524.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 393.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 6756.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 343.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 149.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 339.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ ^1.internal_type not in {IfStatement, JSXOpeningElement}<br>	∧ ^1.roles not in {CALLEE, DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 217.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles in {IDENTIFIER} and not in {KEY}<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement, JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 283.` |
| 17 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type = JSXIdentifier<br>⇒ y = "<br>Confidence: 0.996. Support: 141.` |
| 18 | `  -1.internal_type = StringLiteral<br>	∧ -3.internal_type not in {JSXIdentifier}<br>⇒ y = '<br>Confidence: 1.000. Support: 1224.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.997. Support: 158.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.978. Support: 201.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {VARIABLE} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 907.` |
| 23 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 714.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 405.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IF} and not in {OPERATOR, VARIABLE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.979. Support: 169.` |
| 26 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 144.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles in {FUNCTION}<br>	∧ -3.reserved not in {;}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 172.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {FUNCTION}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {IF, OPERATOR, VARIABLE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.929. Support: 148.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IDENTIFIER}<br>	∧ +2.reserved = ><br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.949. Support: 265.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=, >}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 14555.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 733.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 316.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.947. Support: 198.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 524.` |
| 35 | `  •••start_col ≥ 14<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {IF}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 166.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {IF}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 433.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -5.roles in {IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {IF}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 245.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = from<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {IF}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 147.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {;, {}<br>	∧ -5.roles not in {FUNCTION, IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {CALLEE}<br>	∧ +2.reserved not in {>}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles in {INCOMPLETE} and not in {ASSIGNMENT, FILE, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 348.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -2.label not in {<newline>}<br>	∧ -3.reserved not in {;, {}<br>	∧ -5.roles not in {FUNCTION, IMPORT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), from, {}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +2.reserved not in {>}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {JSXOpeningElement}<br>	∧ ^1.roles not in {ASSIGNMENT, FILE, IF, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 11981.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.996. Support: 140.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, =}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.978. Support: 1186.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +4.reserved not in {,}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.966. Support: 191.` |
| 44 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 711.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 420.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.961. Support: 168.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles in {FUNCTION}<br>	∧ -3.reserved not in {;}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 182.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {IDENTIFIER} and not in {KEY}<br>	∧ +2.reserved = ><br>	∧ +3.reserved not in {(}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 261.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=, >}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 14388.` |
| 50 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = '<br>Confidence: 1.000. Support: 1021.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {RIGHT}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 311.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 938.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.length ≥ 3<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 694.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 398.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.985. Support: 169.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 189.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles in {FUNCTION}<br>	∧ -3.reserved not in {;}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {DECLARATION} and not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 198.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.length ≤ 1<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {DECLARATION, IF, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 152.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {DECLARATION, FILE, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 198.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {DECLARATION, FILE, IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 13740.` |
| 61 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved = =<br>⇒ y = "<br>Confidence: 0.997. Support: 174.` |
| 62 | `  -1.internal_type = StringLiteral<br>	∧ -2.reserved not in {=}<br>⇒ y = '<br>Confidence: 1.000. Support: 1163.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {:}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.996. Support: 142.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.981. Support: 952.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles in {FUNCTION}<br>	∧ -3.reserved not in {;}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 178.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IF} and not in {DECLARATION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.975. Support: 141.` |
| 67 | `  •••start_col ≥ 13<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ ^1.roles in {IF} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 134.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ +3.reserved = ><br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 207.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 13945.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.999. Support: 647.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type = JSXIdentifier<br>	∧ +1.roles in {LITERAL}<br>⇒ y = "<br>Confidence: 0.997. Support: 174.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 681.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -2.internal_type not in {JSXIdentifier}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 292.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ +4.reserved not in {,}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.959. Support: 160.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -3.length ≥ 2<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 904.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 668.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -3.roles in {BINARY}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 191.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.length ≥ 3<br>	∧ -3.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 703.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 6667.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ -3.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 385.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 280.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, VARIABLE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 159.` |
| 83 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved = .<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {CALLEE, DECLARATION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 237.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles in {IDENTIFIER} and not in {KEY, LITERAL}<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 272.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ +2.reserved not in {=, >}<br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 9266.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 0.997. Support: 161.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {INCOMPLETE}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 0.991. Support: 160.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -3.length ≥ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 926.` |
| 89 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 522.` |
| 90 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {ConditionalExpression, VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 6664.` |
| 91 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.927. Support: 376.` |
| 92 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.997. Support: 154.` |
| 93 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 274.` |
| 94 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ ^1.internal_type not in {IfStatement, JSXOpeningElement, VariableDeclarator}<br>	∧ ^1.roles not in {CALLEE, DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 227.` |
| 95 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles in {IDENTIFIER} and not in {KEY}<br>	∧ +2.reserved = ><br>	∧ ^1.internal_type not in {ConditionalExpression, IfStatement, JSXOpeningElement, VariableDeclarator}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 317.` |
| 96 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.reserved = ><br>	∧ +3.length ≤ 6<br>	∧ ^1.internal_type not in {IfStatement, JSXOpeningElement}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 155.` |
| 97 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, const}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.roles not in {KEY}<br>	∧ +2.reserved not in {>}<br>	∧ ^1.internal_type not in {ConditionalExpression, File, IfStatement, JSXOpeningElement, VariableDeclarator}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 9235.` |
| 98 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +4.reserved not in {,}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.954. Support: 162.` |
| 99 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -3.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.971. Support: 942.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IF} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.971. Support: 157.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 364.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = from<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {IF, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 229.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {from}<br>	∧ +1.roles not in {IDENTIFIER}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 416.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;, {}<br>	∧ -4.diff_offset ≥ 13<br>	∧ -5.reserved not in {;}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {CALLEE}<br>	∧ +2.reserved not in {=, >}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {ASSIGNMENT, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 391.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, {}<br>	∧ -3.reserved not in {;, {}<br>	∧ -5.reserved not in {;}<br>	∧ -5.roles not in {FUNCTION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), from, {}<br>	∧ +1.roles not in {CALLEE}<br>	∧ +2.reserved not in {=, >}<br>	∧ +3.reserved not in {(}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {ASSIGNMENT, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 11757.` |
| 106 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.952. Support: 218.` |
| 107 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -3.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {EXPRESSION, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 653.` |
| 108 | `  -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved = =<br>	∧ ^1.roles not in {DECLARATION, IF, INCOMPLETE, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 164.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = /<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 212.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :, ;}<br>	∧ -3.reserved not in {;}<br>	∧ -5.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {/}<br>	∧ +1.roles not in {MAP}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 13791.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.963636363636363, "max_conf": 0.9995915293693542, "max_support": 14555, "min_conf": 0.9219188690185547, "min_support": 134, "num_rules": 110}}
```
</details>
