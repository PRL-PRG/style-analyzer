# Model report for file:///tmp/top-repos-quality-repos-1xc8eazm/xchange.git HEAD 845264b7f9de5d3be122c06c4bf2e6e5aae69222

### Dump

```json
{'created_at': '2021-08-21 03:39:38',
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
 'size': '18.1 kB',
 'tags': [],
 'uuid': 'ec145f6a-5fa3-4b47-8d66-ec160123bffd',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-1xc8eazm/xchange.git 845264b7f9de5d3be122c06c4bf2e6e5aae69222

# javascript
82 rules, avg.len. 7.0
## train
PPCR: 0.951411
### report
macro
{'f1-score': 0.6533726567727108,
 'precision': 0.6829466730109972,
 'recall': 0.6331665733759413,
 'support': 15038}
micro
{'f1-score': 0.9045085782683867,
 'precision': 0.9045085782683867,
 'recall': 0.9045085782683867,
 'support': 15038}
weighted
{'f1-score': 0.8916281488883973,
 'precision': 0.8815692361987026,
 'recall': 0.9045085782683867,
 'support': 15038}
### report_full
macro
{'f1-score': 0.636145651341834,
 'precision': 0.6829466730109972,
 'recall': 0.6005871832124894,
 'support': 15806}
micro
{'f1-score': 0.8819867721436908,
 'precision': 0.9045085782683867,
 'recall': 0.8605592812855878,
 'support': 15806}
weighted
{'f1-score': 0.8666641704991597,
 'precision': 0.8763337519249673,
 'recall': 0.8605592812855878,
 'support': 15806}
## test
PPCR: 0.971490
### report
macro
{'f1-score': 0.6046290767652938,
 'precision': 0.652716021805611,
 'recall': 0.5714428543289304,
 'support': 2249}
micro
{'f1-score': 0.8212538906180524,
 'precision': 0.8212538906180524,
 'recall': 0.8212538906180524,
 'support': 2249}
weighted
{'f1-score': 0.8121160129978774,
 'precision': 0.8099096662086636,
 'recall': 0.8212538906180524,
 'support': 2249}
### report_full
macro
{'f1-score': 0.5867926520309946,
 'precision': 0.652716021805611,
 'recall': 0.541033529612255,
 'support': 2315}
micro
{'f1-score': 0.8093777388255917,
 'precision': 0.8212538906180524,
 'recall': 0.7978401727861771,
 'support': 2315}
weighted
{'f1-score': 0.7982325147434584,
 'precision': 0.807880620304903,
 'recall': 0.7978401727861771,
 'support': 2315}
```

## javascript
### Summary
47 rules, avg.len. 7.2

| | |
|-|-|
|Min support|153|
|Max support|5584|
|Min confidence|0.920131266117096|
|Max confidence|0.9974226951599121|

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
                     'min_samples_leaf': 105,
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
| 1 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.977. Support: 199.` |
| 2 | `  -1.reserved not in {;}<br>	∧ -1.roles in {FUNCTION} and not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 180.` |
| 3 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.947. Support: 179.` |
| 4 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 176.` |
| 5 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 166.` |
| 6 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.958. Support: 153.` |
| 7 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, var, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.922. Support: 5584.` |
| 8 | `  -1.reserved = ;<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.952. Support: 198.` |
| 9 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.941. Support: 415.` |
| 10 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.948. Support: 184.` |
| 11 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.968. Support: 172.` |
| 12 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 3533.` |
| 13 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.945. Support: 462.` |
| 14 | `  -1.reserved = {<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.964. Support: 266.` |
| 15 | `  -1.reserved not in {;, {}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 295.` |
| 16 | `  -1.reserved = var<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 194.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.975. Support: 222.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {FUNCTION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 166.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 185.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, var}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 175.` |
| 21 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 177.` |
| 22 | `  -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 3532.` |
| 23 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.956. Support: 172.` |
| 24 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 600.` |
| 25 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 466.` |
| 26 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.966. Support: 220.` |
| 27 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 177.` |
| 28 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.932. Support: 212.` |
| 29 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.930. Support: 424.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.975. Support: 182.` |
| 32 | `  -1.internal_type = CommentLine<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.967. Support: 165.` |
| 33 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 3425.` |
| 34 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ +1.length ≥ 2<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 168.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.959. Support: 181.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;, var, {}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.969. Support: 175.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {DECLARATION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 3463.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 677.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,}<br>	∧ -1.roles not in {COMMENT}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.969. Support: 474.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 244.` |
| 41 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ -1.roles not in {COMMENT, EXPRESSION}<br>	∧ +1.reserved not in {), ;}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ ^1.roles not in {DECLARATION, EXPRESSION, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 169.` |
| 42 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.930. Support: 422.` |
| 43 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ␣<br>Confidence: 0.977. Support: 281.` |
| 44 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ +1.reserved not in {{}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.943. Support: 256.` |
| 45 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = var<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 189.` |
| 46 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.920. Support: 457.` |
| 47 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {DECLARATION} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 170.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.212765957446808, "max_conf": 0.9974226951599121, "max_support": 5584, "min_conf": 0.920131266117096, "min_support": 153, "num_rules": 47}}
```
</details>
