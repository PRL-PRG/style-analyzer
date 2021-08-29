# Model report for file:///tmp/top-repos-quality-repos-rnviw1oa/universal-data-tool.git HEAD 1e6d01c99ba437a4242f336ba0504ea352f73ed0

### Dump

```json
{'created_at': '2021-08-29 13:41:04',
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
 'size': '22.7 kB',
 'tags': [],
 'uuid': 'f232585d-7f38-450f-b247-9eb3b7545242',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-rnviw1oa/universal-data-tool.git 1e6d01c99ba437a4242f336ba0504ea352f73ed0

# javascript
239 rules, avg.len. 9.9
## train
PPCR: 0.957377
### report
macro
{'f1-score': 0.5733267866205547,
 'precision': 0.6023803327345008,
 'recall': 0.5526169034392616,
 'support': 73628}
micro
{'f1-score': 0.9239419785950997,
 'precision': 0.9239419785950997,
 'recall': 0.9239419785950997,
 'support': 73628}
weighted
{'f1-score': 0.9181308486468465,
 'precision': 0.9158789610395272,
 'recall': 0.9239419785950997,
 'support': 73628}
### report_full
macro
{'f1-score': 0.5541026973442322,
 'precision': 0.6023803327345008,
 'recall': 0.52030555247609,
 'support': 76906}
micro
{'f1-score': 0.9038223922834709,
 'precision': 0.9239419785950997,
 'recall': 0.8845603724026734,
 'support': 76906}
weighted
{'f1-score': 0.892496606972326,
 'precision': 0.9070657858625263,
 'recall': 0.8845603724026734,
 'support': 76906}
## test
PPCR: 0.955204
### report
macro
{'f1-score': 0.5755641230107021,
 'precision': 0.6017507026776916,
 'recall': 0.5561614403200639,
 'support': 17208}
micro
{'f1-score': 0.9225941422594143,
 'precision': 0.9225941422594143,
 'recall': 0.9225941422594143,
 'support': 17208}
weighted
{'f1-score': 0.9168147542675921,
 'precision': 0.9143356511106279,
 'recall': 0.9225941422594143,
 'support': 17208}
### report_full
macro
{'f1-score': 0.5544258889548743,
 'precision': 0.6017507026776916,
 'recall': 0.5211509621128791,
 'support': 18015}
micro
{'f1-score': 0.9014564347159527,
 'precision': 0.9225941422594143,
 'recall': 0.8812656119900083,
 'support': 18015}
weighted
{'f1-score': 0.8890614443058692,
 'precision': 0.9038848469831718,
 'recall': 0.8812656119900083,
 'support': 18015}
```

## javascript
### Summary
135 rules, avg.len. 9.2

| | |
|-|-|
|Min support|127|
|Max support|10863|
|Min confidence|0.9203821420669556|
|Max confidence|0.9998576641082764|

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
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 1.000. Support: 3513.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>⇒ y = "<br>Confidence: 1.000. Support: 2316.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 493.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 455.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {{}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 231.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.946. Support: 698.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +4.reserved = }<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 318.` |
| 8 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.953. Support: 161.` |
| 9 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 254.` |
| 10 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.950. Support: 1699.` |
| 11 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.diff_col ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 364.` |
| 12 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 169.` |
| 13 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.988. Support: 452.` |
| 14 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 281.` |
| 15 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 7006.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.972. Support: 741.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 10863.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 244.` |
| 19 | `  -1.diff_col ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label in {<newline>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.941. Support: 834.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.reserved = =<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 211.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, >}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.label not in {<newline>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles in {IF} and not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 194.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -4.length ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 2277.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 254.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -3.reserved not in {>}<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +2.reserved = =<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 132.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 1390.` |
| 26 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.934. Support: 175.` |
| 27 | `  •••start_col ≥ 20<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {OPERATOR} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 354.` |
| 28 | `  •••start_col ≤ 19<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 1575.` |
| 29 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_col ≥ 4<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {BLOCK, DECLARATION}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 585.` |
| 30 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.roles not in {FILE}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 6502.` |
| 31 | `  •••start_col ≤ 18<br>	∧ -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.980. Support: 616.` |
| 32 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = if<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 262.` |
| 33 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 236.` |
| 34 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, if}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {IF} and not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 216.` |
| 35 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IF, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 269.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.978. Support: 301.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<newline>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.996. Support: 134.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2361.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2005.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.internal_type = JSXIdentifier<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 499.` |
| 41 | `  •••start_col ≤ 10<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≥ 7<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 456.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 315.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {BLOCK, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.959. Support: 2355.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 731.` |
| 45 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :, {}<br>	∧ -1.length ≤ 1<br>	∧ -3.internal_type not in {Identifier}<br>	∧ -4.diff_offset ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {=}<br>	∧ +4.length ≥ 12<br>	∧ +5.reserved not in {,}<br>	∧ ^1.internal_type not in {CallExpression, JSXElement}<br>	∧ ^1.roles not in {DECLARATION, LIST, OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.971. Support: 504.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 1.000. Support: 1189.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles not in {IDENTIFIER}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 526.` |
| 48 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 186.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 1151.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {LITERAL} and not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.966. Support: 309.` |
| 51 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 540.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 279.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 203.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 156.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, >, import}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {IF} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.960. Support: 215.` |
| 56 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, >, [, import}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, [, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 7002.` |
| 57 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -4.length ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 35<br>	∧ +3.internal_type = Identifier<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 1225.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -4.length ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.length ≤ 35<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 388.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {:}<br>	∧ -2.label in {<space>}<br>	∧ -4.length ≤ 2<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.internal_type not in {Identifier}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 375.` |
| 60 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≥ 3<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.920. Support: 157.` |
| 61 | `  •••start_col ≤ 20<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 1634.` |
| 62 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≤ 2<br>	∧ -2.roles not in {MAP}<br>	∧ -5.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ␣<br>Confidence: 0.935. Support: 176.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.973. Support: 426.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -1.length ≤ 2<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ␣<br>Confidence: 0.948. Support: 299.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.length ≤ 2<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 6794.` |
| 66 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ -4.reserved not in {=}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {}}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 3851.` |
| 67 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 258.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles in {IDENTIFIER}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.984. Support: 1382.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -2.label in {<space>}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 401.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved = :<br>	∧ -2.label not in {<space>}<br>	∧ -4.length ≤ 1<br>	∧ +1.internal_type = StringLiteral<br>	∧ +3.roles not in {IDENTIFIER}<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INITIALIZATION}<br>⇒ y = ␣<br>Confidence: 0.947. Support: 423.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 2379.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = JSXElement<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1966.` |
| 73 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.921. Support: 1254.` |
| 74 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 482.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≤ 6<br>	∧ -3.reserved not in {,}<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.964. Support: 1017.` |
| 76 | `  •••start_col ≤ 18<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type = BlockStatement<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 320.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement, JSXElement}<br>	∧ ^1.roles not in {IDENTIFIER, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 2462.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 752.` |
| 79 | `  •••start_col ≥ 11<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = }<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER, LIST}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 299.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 127.` |
| 81 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {IDENTIFIER}<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 239.` |
| 82 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +4.internal_type = StringLiteral<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.989. Support: 318.` |
| 83 | `  -1.diff_offset ≥ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = (<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 593.` |
| 84 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved = ><br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 262.` |
| 85 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 189.` |
| 86 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.reserved = =<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 165.` |
| 87 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = const<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 164.` |
| 88 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, >, import}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ ^1.roles in {IF} and not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 231.` |
| 89 | `  •••start_col ≤ 15<br>	∧ -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.978. Support: 1491.` |
| 90 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.roles not in {MAP}<br>	∧ -3.diff_offset ≥ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {INCOMPLETE} and not in {BLOCK, DECLARATION, MODULE}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 588.` |
| 91 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.diff_col ≥ 20<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION, INCOMPLETE, MODULE}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 183.` |
| 92 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION, MODULE, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 6474.` |
| 93 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {), >}<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.998. Support: 313.` |
| 94 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.958. Support: 153.` |
| 95 | `  •••start_col ≤ 19<br>	∧ -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.979. Support: 1525.` |
| 96 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.roles not in {MAP}<br>	∧ -5.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 198.` |
| 97 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.957. Support: 434.` |
| 98 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.label in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.943. Support: 271.` |
| 99 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,}<br>	∧ -2.label not in {<space>}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {Program}<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.934. Support: 6951.` |
| 100 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>, }}<br>	∧ ^1.roles in {INCOMPLETE} and not in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 3825.` |
| 101 | `  -1.diff_col ≤ 4<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = {<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 258.` |
| 102 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.diff_line ≥ 1<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = "<br>Confidence: 0.997. Support: 159.` |
| 103 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≤ 6<br>	∧ -3.internal_type not in {JSXIdentifier}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.926. Support: 1050.` |
| 104 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -1.length ≥ 3<br>	∧ -4.diff_line ≥ 1<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 321.` |
| 105 | `  •••start_col ≥ 11<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles in {STRING}<br>	∧ +5.reserved = ,<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, LIST, QUALIFIED}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 178.` |
| 106 | `  •••start_col ≥ 11<br>	∧ -1.diff_col ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :, {}<br>	∧ -1.length ≤ 2<br>	∧ -4.diff_col ≥ 7<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {VALUE}<br>	∧ +1.length ≥ 2<br>	∧ +3.reserved not in {=}<br>	∧ +4.length ≥ 10<br>	∧ +5.reserved not in {,}<br>	∧ ^1.internal_type not in {CallExpression, JSXElement, ObjectExpression}<br>	∧ ^1.roles not in {DECLARATION, LIST, OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.943. Support: 548.` |
| 107 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ,<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 141.` |
| 108 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_offset ≥ 5<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, =}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.930. Support: 338.` |
| 109 | `  -1.internal_type not in {StringLiteral}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {,, =}<br>	∧ +1.length ≤ 1<br>	∧ +2.length ≥ 2<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.986. Support: 463.` |
| 110 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 308.` |
| 111 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = )<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.935. Support: 421.` |
| 112 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = import<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 214.` |
| 113 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, >, import}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {(, =, [, }}<br>	∧ +1.length ≤ 1<br>	∧ +2.reserved not in {>}<br>	∧ +3.length ≥ 1<br>	∧ ^1.roles not in {DECLARATION, IF, OPERATOR, STATEMENT}<br>	∧ ^2.internal_type not in {ObjectExpression}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 6913.` |
| 114 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<newline>, <space>}<br>	∧ -2.diff_offset ≥ 3<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.internal_type not in {CallExpression}<br>	∧ ^1.roles not in {INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.955. Support: 2233.` |
| 115 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.roles in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 502.` |
| 116 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -3.diff_col ≥ 7<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ -5.diff_offset ≤ 11<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 416.` |
| 117 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ -3.diff_col ≤ 6<br>	∧ -3.roles not in {INCOMPLETE}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.922. Support: 1087.` |
| 118 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ -2.label in {<newline>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles in {BLOCK} and not in {INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 304.` |
| 119 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {BLOCK, INCOMPLETE, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 2541.` |
| 120 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.964. Support: 774.` |
| 121 | `  •••start_col ≥ 11<br>	∧ -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +4.reserved = }<br>	∧ ^1.internal_type not in {JSXElement}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 312.` |
| 122 | `  •••start_col ≥ 11<br>	∧ -1.diff_offset ≤ 1<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, :, {}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved = :<br>	∧ +4.reserved not in {}}<br>	∧ ^1.internal_type not in {CallExpression, JSXElement}<br>	∧ ^1.roles not in {DECLARATION, LIST, OPERATOR, QUALIFIED}<br>	∧ ^2.roles not in {BLOCK}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 1050.` |
| 123 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ><br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 358.` |
| 124 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = =<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 193.` |
| 125 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {=, >}<br>	∧ -4.diff_line ≥ 1<br>	∧ -4.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 133.` |
| 126 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +2.roles in {EXPRESSION}<br>	∧ +4.roles in {STRING}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.980. Support: 322.` |
| 127 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ⏎<br>Confidence: 0.944. Support: 171.` |
| 128 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {BlockStatement}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.929. Support: 344.` |
| 129 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.954. Support: 1715.` |
| 130 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = :<br>	∧ -2.diff_col ≥ 3<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 355.` |
| 131 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :}<br>	∧ -2.diff_col ≥ 3<br>	∧ -3.diff_offset ≤ 6<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 171.` |
| 132 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.975. Support: 464.` |
| 133 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, {}<br>	∧ -2.label in {<space>}<br>	∧ -3.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ␣<br>Confidence: 0.928. Support: 300.` |
| 134 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {), ,, :}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {BLOCK, DECLARATION}<br>	∧ ^2.internal_type not in {File}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 6743.` |
| 135 | `  •••start_col ≤ 24<br>	∧ -1.diff_col ≥ 5<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE}<br>⇒ y = ␣<br>Confidence: 0.957. Support: 667.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 9.237037037037037, "max_conf": 0.9998576641082764, "max_support": 10863, "min_conf": 0.9203821420669556, "min_support": 127, "num_rules": 135}}
```
</details>
