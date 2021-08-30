# Model report for file:///tmp/top-repos-quality-repos-th0k9xmq/codeceptjs.git HEAD 42717c09173fdf8276166c7918b0bbd5c9f1055a

### Dump

```json
{'created_at': '2021-08-30 00:44:42',
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
 'size': '23.0 kB',
 'tags': [],
 'uuid': '2b29e0b0-4413-4370-be59-3cca25dc6816',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-th0k9xmq/codeceptjs.git 42717c09173fdf8276166c7918b0bbd5c9f1055a

# javascript
213 rules, avg.len. 10.2
## train
PPCR: 0.992804
### report
macro
{'f1-score': 0.6453024729315662,
 'precision': 0.6616248442534027,
 'recall': 0.6331450220570213,
 'support': 205695}
micro
{'f1-score': 0.9723328228688106,
 'precision': 0.9723328228688106,
 'recall': 0.9723328228688106,
 'support': 205695}
weighted
{'f1-score': 0.9707533657103142,
 'precision': 0.9701028333228667,
 'recall': 0.9723328228688106,
 'support': 205695}
### report_full
macro
{'f1-score': 0.6390062129819878,
 'precision': 0.6616248442534027,
 'recall': 0.6223507829586996,
 'support': 207186}
micro
{'f1-score': 0.9688215248461421,
 'precision': 0.9723328228688106,
 'recall': 0.9653354956415974,
 'support': 207186}
weighted
{'f1-score': 0.9669293515677678,
 'precision': 0.9698269388626491,
 'recall': 0.9653354956415974,
 'support': 207186}
## test
PPCR: 0.994212
### report
macro
{'f1-score': 0.6414070661979276,
 'precision': 0.6638174351639357,
 'recall': 0.6266677991666885,
 'support': 48781}
micro
{'f1-score': 0.9734527787458231,
 'precision': 0.9734527787458231,
 'recall': 0.9734527787458231,
 'support': 48781}
weighted
{'f1-score': 0.9720401911843587,
 'precision': 0.9723469103470473,
 'recall': 0.9734527787458231,
 'support': 48781}
### report_full
macro
{'f1-score': 0.6364938449988167,
 'precision': 0.6638174351639357,
 'recall': 0.618229436015548,
 'support': 49065}
micro
{'f1-score': 0.9706273123070949,
 'precision': 0.9734527787458231,
 'recall': 0.9678182003464791,
 'support': 49065}
weighted
{'f1-score': 0.9689794609279694,
 'precision': 0.9721304312475034,
 'recall': 0.9678182003464791,
 'support': 49065}
```

## javascript
### Summary
154 rules, avg.len. 10.4

| | |
|-|-|
|Min support|125|
|Max support|44967|
|Min confidence|0.9220430254936218|
|Max confidence|0.9997517466545105|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.990. Support: 7305.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1205.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 970.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.977. Support: 7607.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.963. Support: 3633.` |
| 6 | `  •••start_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>⇒ y = ⏎<br>Confidence: 0.997. Support: 179.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>⇒ y = ␣<br>Confidence: 0.999. Support: 6483.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 817.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.999. Support: 353.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 40760.` |
| 11 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 747.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 8859.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 304.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.989. Support: 3876.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved not in {,}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.960. Support: 845.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.985. Support: 366.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 4251.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ∅<br>Confidence: 0.999. Support: 338.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.988. Support: 3934.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.991. Support: 2265.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 1859.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1186.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1006.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.982. Support: 995.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type = CommentBlock<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.984. Support: 729.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles in {STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 228.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.947. Support: 936.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 298.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 2402.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = AssignmentPattern<br>⇒ y = ␣<br>Confidence: 0.992. Support: 183.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +4.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 285.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {'}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 161.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.length ≥ 4<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 323.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,, :, ;, >, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.977. Support: 850.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,, :, ;, >, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 44658.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +5.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 1027.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_offset ≤ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 327.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.992. Support: 4077.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 827.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.999. Support: 354.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 40784.` |
| 42 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 743.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 8822.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.970. Support: 858.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles in {EXPRESSION} and not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 3389.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.991. Support: 2335.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 188.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 2088.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type = CommentBlock<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.978. Support: 759.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 210.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.947. Support: 948.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 318.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2472.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +4.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 312.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ ^1.internal_type = AssignmentPattern<br>⇒ y = ␣<br>Confidence: 0.975. Support: 177.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = '<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 170.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.length ≥ 4<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.999. Support: 360.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ∅<br>Confidence: 0.986. Support: 809.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 44544.` |
| 60 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 989.` |
| 61 | `  •••start_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {STRING}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 182.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -5.diff_offset ≤ 12<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 399.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 945.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.985. Support: 781.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 218.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>⇒ y = ⏎⏎<br>Confidence: 0.945. Support: 975.` |
| 67 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 313.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2379.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.length ≥ 4<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 363.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 880.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 44733.` |
| 72 | `  •••start_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≥ 2<br>	∧ +3.length ≥ 7<br>⇒ y = ⏎⏎<br>Confidence: 0.971. Support: 648.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION, VALUE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.998. Support: 333.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {VALUE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 730.` |
| 75 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 710.` |
| 76 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 8829.` |
| 77 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP, VALUE}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 252.` |
| 78 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +3.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 164.` |
| 79 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles not in {MAP}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.990. Support: 4116.` |
| 80 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.980. Support: 376.` |
| 81 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.roles not in {LITERAL}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 4184.` |
| 82 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ∅<br>Confidence: 0.999. Support: 385.` |
| 83 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>⇒ y = ␣<br>Confidence: 0.990. Support: 4104.` |
| 84 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.993. Support: 2238.` |
| 85 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ∅<br>Confidence: 0.938. Support: 184.` |
| 86 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 191.` |
| 87 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 2132.` |
| 88 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1159.` |
| 89 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 973.` |
| 90 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.931. Support: 1102.` |
| 91 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = CommentBlock<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.990. Support: 749.` |
| 92 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 216.` |
| 93 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎⏎<br>Confidence: 0.951. Support: 985.` |
| 94 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 329.` |
| 95 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2370.` |
| 96 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +4.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.995. Support: 309.` |
| 97 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ ^1.internal_type = AssignmentPattern<br>⇒ y = ␣<br>Confidence: 0.991. Support: 165.` |
| 98 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.length ≥ 4<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ␣<br>Confidence: 0.999. Support: 364.` |
| 99 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ∅<br>Confidence: 0.980. Support: 869.` |
| 100 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement, VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 44374.` |
| 101 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 1033.` |
| 102 | `  •••start_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {STRING}<br>	∧ ^1.roles not in {FILE}<br>⇒ y = ⏎<br>Confidence: 0.993. Support: 204.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -5.diff_offset ≥ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +4.reserved = }<br>⇒ y = ␣<br>Confidence: 0.996. Support: 140.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ ^1.internal_type = AssignmentPattern<br>⇒ y = ␣<br>Confidence: 0.997. Support: 152.` |
| 105 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {'}<br>	∧ -1.reserved not in {,, :, ;, >, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 892.` |
| 106 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 10<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 309.` |
| 107 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +2.reserved not in {,}<br>	∧ +4.reserved = ,<br>	∧ ^1.roles in {LITERAL}<br>⇒ y = ⏎<br>Confidence: 0.952. Support: 859.` |
| 108 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ␣<br>Confidence: 0.992. Support: 2336.` |
| 109 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, =, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type = VariableDeclarator<br>⇒ y = ∅<br>Confidence: 0.922. Support: 186.` |
| 110 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 173.` |
| 111 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {{}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 1139.` |
| 112 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type = CommentBlock<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.983. Support: 673.` |
| 113 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 205.` |
| 114 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.926. Support: 939.` |
| 115 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 325.` |
| 116 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 2380.` |
| 117 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +4.reserved = (<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 252.` |
| 118 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 3<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = BlockStatement<br>⇒ y = ∅<br>Confidence: 0.973. Support: 903.` |
| 119 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.internal_type not in {CommentBlock}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {BlockStatement}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 44609.` |
| 120 | `  •••start_col ≤ 8<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 7<br>	∧ +3.roles not in {STRING}<br>⇒ y = ⏎<br>Confidence: 0.992. Support: 177.` |
| 121 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +3.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 239.` |
| 122 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved = }<br>	∧ +3.roles not in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 184.` |
| 123 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 730.` |
| 124 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 8588.` |
| 125 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.roles in {COMMENT} and not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 729.` |
| 126 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {STATEMENT} and not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 214.` |
| 127 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {COMMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2395.` |
| 128 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {.}<br>	∧ +1.length ≥ 4<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 340.` |
| 129 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {COMMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 3<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 912.` |
| 130 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {COMMENT, MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 44957.` |
| 131 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type = StringLiteral<br>	∧ +5.roles not in {STRING}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 1007.` |
| 132 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP, VALUE}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 246.` |
| 133 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +3.roles in {VALUE}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 145.` |
| 134 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION, VALUE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>⇒ y = ␣<br>Confidence: 0.999. Support: 358.` |
| 135 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, {}<br>	∧ -1.roles in {EXPRESSION} and not in {VALUE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 777.` |
| 136 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^2.roles in {SCOPE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 205.` |
| 137 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.932. Support: 975.` |
| 138 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved not in {)}<br>	∧ -5.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +4.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.964. Support: 125.` |
| 139 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +3.roles in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 220.` |
| 140 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.roles in {MAP}<br>	∧ +2.reserved = }<br>	∧ +3.roles not in {MAP}<br>	∧ +4.reserved not in {,}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 180.` |
| 141 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -5.reserved = const<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 2014.` |
| 142 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 171.` |
| 143 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 2108.` |
| 144 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 1058.` |
| 145 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles in {COMMENT} and not in {MAP}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.986. Support: 734.` |
| 146 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ⏎⏎<br>Confidence: 0.951. Support: 949.` |
| 147 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.998. Support: 312.` |
| 148 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<-space>}<br>	∧ -2.roles not in {COMMENT, MAP}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 2295.` |
| 149 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {VARIABLE}<br>⇒ y = ␣<br>Confidence: 0.949. Support: 301.` |
| 150 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, =, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {COMMENT, MAP}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.roles in {VARIABLE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.974. Support: 174.` |
| 151 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +4.reserved = (<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ⏎<br>Confidence: 0.982. Support: 254.` |
| 152 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {COMMENT, MAP}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ +1.length ≤ 3<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 581.` |
| 153 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, >, if, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<-space>}<br>	∧ -2.roles not in {COMMENT, MAP}<br>	∧ -5.reserved not in {const}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, {}<br>	∧ ^1.internal_type not in {IfStatement}<br>	∧ ^1.roles not in {OPERATOR, VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 44967.` |
| 154 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;, if}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ -2.reserved = )<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = .<br>	∧ +4.reserved = (<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎<br>Confidence: 0.991. Support: 288.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.383116883116884, "max_conf": 0.9997517466545105, "max_support": 44967, "min_conf": 0.9220430254936218, "min_support": 125, "num_rules": 154}}
```
</details>
