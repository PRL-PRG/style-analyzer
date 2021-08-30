# Model report for file:///tmp/top-repos-quality-repos-ntyhw93u/microstates.git HEAD df4ae5c5984e296e0481b823419df7ee578b64d4

### Dump

```json
{'created_at': '2021-08-29 13:07:01',
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
 'size': '18.8 kB',
 'tags': [],
 'uuid': '4b633367-3189-4df4-8dc0-0f88fc7a0cad',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-ntyhw93u/microstates.git df4ae5c5984e296e0481b823419df7ee578b64d4

# javascript
108 rules, avg.len. 7.8
## train
PPCR: 0.925579
### report
macro
{'f1-score': 0.7330850112494204,
 'precision': 0.7763101272324926,
 'recall': 0.7025555518617982,
 'support': 20969}
micro
{'f1-score': 0.9092469836425199,
 'precision': 0.9092469836425199,
 'recall': 0.9092469836425199,
 'support': 20969}
weighted
{'f1-score': 0.9047364416831966,
 'precision': 0.9035809993657622,
 'recall': 0.9092469836425199,
 'support': 20969}
### report_full
macro
{'f1-score': 0.6818181809827998,
 'precision': 0.7763101272324926,
 'recall': 0.6335201935689507,
 'support': 22655}
micro
{'f1-score': 0.8741059966990647,
 'precision': 0.9092469836425199,
 'recall': 0.8415802251158685,
 'support': 22655}
weighted
{'f1-score': 0.8534611040683986,
 'precision': 0.8824077239427935,
 'recall': 0.8415802251158685,
 'support': 22655}
## test
PPCR: 0.943600
### report
macro
{'f1-score': 0.641059446375939,
 'precision': 0.7078995934239962,
 'recall': 0.6143935011545374,
 'support': 4969}
micro
{'f1-score': 0.8782451197424029,
 'precision': 0.8782451197424029,
 'recall': 0.8782451197424029,
 'support': 4969}
weighted
{'f1-score': 0.8703758795485038,
 'precision': 0.8718288521226589,
 'recall': 0.8782451197424029,
 'support': 4969}
### report_full
macro
{'f1-score': 0.6057395141967163,
 'precision': 0.7078995934239962,
 'recall': 0.56001637800626,
 'support': 5266}
micro
{'f1-score': 0.8527601367855399,
 'precision': 0.8782451197424029,
 'recall': 0.8287124952525636,
 'support': 5266}
weighted
{'f1-score': 0.841549309861917,
 'precision': 0.8697809180896471,
 'recall': 0.8287124952525636,
 'support': 5266}
```

## javascript
### Summary
80 rules, avg.len. 7.5

| | |
|-|-|
|Min support|158|
|Max support|3821|
|Min confidence|0.9204335808753967|
|Max confidence|0.9994775056838989|

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
| 1 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.995. Support: 516.` |
| 2 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2148.` |
| 3 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {EXPRESSION} and not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.970. Support: 384.` |
| 4 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION, MAP}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 957.` |
| 5 | `  -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {EXPRESSION, MAP}<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 779.` |
| 6 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {EXPRESSION, MAP}<br>	∧ +3.reserved not in {;}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {CALL}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 726.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1175.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 365.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles in {DECLARATION, FUNCTION}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 158.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 1883.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ><br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 313.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = ;<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 264.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {>}<br>	∧ ^1.roles in {QUALIFIED}<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 205.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = )<br>	∧ ^2.roles not in {CALL}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 190.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 2419.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 432.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 977.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 322.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.951. Support: 1417.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 312.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 239.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 196.` |
| 23 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, >}<br>	∧ ^2.roles in {STATEMENT} and not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 2168.` |
| 24 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, >}<br>	∧ ^2.roles not in {EXPRESSION, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.990. Support: 360.` |
| 25 | `  -1.reserved = ;<br>	∧ +1.reserved not in {}}<br>	∧ +3.internal_type = StringLiteral<br>⇒ y = ⏎⏎<br>Confidence: 0.942. Support: 268.` |
| 26 | `  -1.reserved = :<br>	∧ +1.roles in {MAP}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 181.` |
| 27 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 2319.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 480.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 3098.` |
| 30 | `  -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1083.` |
| 31 | `  -1.reserved not in {(, ;, {}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 725.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved = )<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 367.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved = ;<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 323.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ><br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 281.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved = ,<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 225.` |
| 36 | `  -1.diff_col ≥ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved = (<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 328.` |
| 37 | `  -1.diff_col ≤ 6<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {,, >}<br>	∧ ^1.roles in {QUALIFIED}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.972. Support: 160.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {BODY} and not in {BINARY, DECLARATION, INITIALIZATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.947. Support: 1986.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {BINARY, BODY, DECLARATION, INITIALIZATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 3599.` |
| 40 | `  -1.reserved = {<br>	∧ -3.diff_col ≤ 7<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.932. Support: 673.` |
| 41 | `  -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1055.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved = ,<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 225.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved not in {,, ;}<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 171.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.reserved = (<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.975. Support: 341.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.968. Support: 238.` |
| 46 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.roles in {QUALIFIED} and not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2361.` |
| 47 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 413.` |
| 48 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type = Identifier<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles in {STATEMENT} and not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 189.` |
| 49 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.internal_type not in {Identifier}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, QUALIFIED}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 2836.` |
| 50 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +2.roles in {ARGUMENT}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.984. Support: 717.` |
| 51 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +3.roles not in {MAP}<br>	∧ ^1.roles in {CALL} and not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 3105.` |
| 52 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≥ 2<br>	∧ +3.roles not in {MAP}<br>	∧ ^1.roles in {IDENTIFIER} and not in {CALL, DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 245.` |
| 53 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.length ≤ 1<br>	∧ +3.roles not in {MAP}<br>	∧ ^1.roles not in {CALL, DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 2555.` |
| 54 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.reserved = )<br>	∧ +1.reserved not in {)}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.925. Support: 367.` |
| 55 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -3.reserved not in {)}<br>	∧ +1.reserved not in {,, ;}<br>	∧ ^1.roles in {QUALIFIED}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 198.` |
| 56 | `  -1.diff_col ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = (<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 327.` |
| 57 | `  -1.diff_col ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {QUALIFIED}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.962. Support: 253.` |
| 58 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION, FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 2029.` |
| 59 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION} and not in {FUNCTION}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 3821.` |
| 60 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 250.` |
| 61 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 238.` |
| 62 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 181.` |
| 63 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 2380.` |
| 64 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 423.` |
| 65 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 2819.` |
| 66 | `  -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 297.` |
| 67 | `  -1.diff_offset ≤ 7<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, >}<br>	∧ ^2.roles not in {EXPRESSION, STATEMENT}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 348.` |
| 68 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles in {IDENTIFIER} and not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 2387.` |
| 69 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.roles in {EXPRESSION} and not in {MAP}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.933. Support: 563.` |
| 70 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -5.diff_line ≤ 1<br>	∧ +1.roles not in {EXPRESSION, MAP}<br>	∧ ^1.roles not in {DECLARATION, IDENTIFIER}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.928. Support: 3180.` |
| 71 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {;}<br>	∧ +2.reserved not in {{}<br>	∧ ^1.roles in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 181.` |
| 72 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = ,<br>	∧ +2.reserved not in {{}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 164.` |
| 73 | `  -1.diff_offset ≥ 3<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = (<br>	∧ +2.reserved not in {{}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 367.` |
| 74 | `  -1.diff_offset ≤ 2<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved = (<br>	∧ +2.reserved not in {{}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.roles not in {IDENTIFIER}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.976. Support: 267.` |
| 75 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.993. Support: 2323.` |
| 76 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 444.` |
| 77 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -2.roles not in {MAP}<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {DECLARATION}<br>	∧ ^2.roles in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.940. Support: 3081.` |
| 78 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ +1.reserved not in {,, >}<br>	∧ ^1.internal_type = MemberExpression<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 220.` |
| 79 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.length ≥ 3<br>	∧ +1.reserved = (<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ∅<br>Confidence: 0.973. Support: 426.` |
| 80 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.length ≤ 2<br>	∧ +1.reserved = (<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^2.roles not in {EXPRESSION}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 283.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.525, "max_conf": 0.9994775056838989, "max_support": 3821, "min_conf": 0.9204335808753967, "min_support": 158, "num_rules": 80}}
```
</details>
