# Model report for file:///tmp/top-repos-quality-repos-fx1fho1c/bilibili-helper-o.git HEAD 06fb124cbc35526eac2b83b4e50fd31f9580d47a

### Dump

```json
{'created_at': '2021-08-29 10:46:15',
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
 'size': '19.6 kB',
 'tags': [],
 'uuid': '74e7a851-a8e6-4261-8358-57022f4d22da',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-fx1fho1c/bilibili-helper-o.git 06fb124cbc35526eac2b83b4e50fd31f9580d47a

# javascript
49 rules, avg.len. 11.2
## train
PPCR: 0.924871
### report
macro
{'f1-score': 0.8044705915000834,
 'precision': 0.8130964949622446,
 'recall': 0.7967032681089147,
 'support': 63411}
micro
{'f1-score': 0.9682231789437163,
 'precision': 0.9682231789437163,
 'recall': 0.9682231789437163,
 'support': 63411}
weighted
{'f1-score': 0.9678650510571659,
 'precision': 0.9676520383906226,
 'recall': 0.9682231789437163,
 'support': 63411}
### report_full
macro
{'f1-score': 0.7230032310642697,
 'precision': 0.8130964949622446,
 'recall': 0.6583068706867282,
 'support': 68562}
micro
{'f1-score': 0.9304327400301576,
 'precision': 0.9682231789437163,
 'recall': 0.8954814620343631,
 'support': 68562}
weighted
{'f1-score': 0.9265807361762513,
 'precision': 0.9642121793756616,
 'recall': 0.8954814620343631,
 'support': 68562}
## test
PPCR: 0.925677
### report
macro
{'f1-score': 0.6950123079554297,
 'precision': 0.7152903100447185,
 'recall': 0.6830246806422539,
 'support': 16515}
micro
{'f1-score': 0.9269149258250077,
 'precision': 0.9269149258250076,
 'recall': 0.9269149258250076,
 'support': 16515}
weighted
{'f1-score': 0.9254778924579168,
 'precision': 0.9313727554104411,
 'recall': 0.9269149258250076,
 'support': 16515}
### report_full
macro
{'f1-score': 0.6384175394705776,
 'precision': 0.7152903100447185,
 'recall': 0.5876000176802739,
 'support': 17841}
micro
{'f1-score': 0.8911398300151356,
 'precision': 0.9269149258250076,
 'recall': 0.858023653382658,
 'support': 17841}
weighted
{'f1-score': 0.8864435406158364,
 'precision': 0.9273429724141518,
 'recall': 0.858023653382658,
 'support': 17841}
```

## javascript
### Summary
31 rules, avg.len. 10.3

| | |
|-|-|
|Min support|104|
|Max support|10115|
|Min confidence|0.9287819266319275|
|Max confidence|0.9997251033782959|

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
| 1 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.990. Support: 10115.` |
| 2 | `  •••start_line ≤ 249<br>	∧ -1.label in {<space>}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.976. Support: 104.` |
| 3 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 2180.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 1421.` |
| 5 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved = =<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 154.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.938. Support: 138.` |
| 7 | `  -1.reserved = ;<br>	∧ +1.reserved = }<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.953. Support: 783.` |
| 8 | `  -1.reserved = ;<br>	∧ +1.reserved = import<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, OPERATOR}<br>	∧ ^2.roles not in {RIGHT}<br>⇒ y = ⏎<br>Confidence: 0.998. Support: 204.` |
| 9 | `  •••start_line ≤ 254<br>	∧ -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {;}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.964. Support: 1305.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {IF}<br>⇒ y = '<br>Confidence: 0.966. Support: 934.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 856.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = ?<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 167.` |
| 13 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {?}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = ClassDeclaration<br>⇒ y = ␣<br>Confidence: 0.996. Support: 119.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {?}<br>	∧ +1.roles not in {LITERAL}<br>	∧ +2.roles in {PATHNAME}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.996. Support: 118.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, ?}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.988. Support: 8170.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 3090.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 1819.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {INCOMPLETE} and not in {BLOCK, OPERATOR}<br>	∧ ^2.roles not in {MODULE}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 2520.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≥ 7<br>	∧ +1.roles not in {KEY, LITERAL}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.961. Support: 293.` |
| 20 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = {<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -3.diff_col ≤ 6<br>	∧ +1.roles not in {LITERAL, MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺␣⁺␣⁺<br>Confidence: 0.929. Support: 1018.` |
| 21 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.979. Support: 801.` |
| 22 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 596.` |
| 23 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), >}<br>	∧ +1.roles in {COMMENT} and not in {LITERAL}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 583.` |
| 24 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = }<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.932. Support: 495.` |
| 25 | `  •••start_col ≥ 56<br>	∧ -1.diff_col ≥ 8<br>	∧ -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 292.` |
| 26 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ,, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), >, }}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {LITERAL} and not in {INCOMPLETE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.968. Support: 235.` |
| 27 | `  -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 212.` |
| 28 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≥ 7<br>	∧ -2.reserved not in {)}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, >, }}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ ^1.internal_type not in {ClassBody, File, IfStatement, MemberExpression}<br>	∧ ^1.roles not in {INCOMPLETE, LITERAL, OPERATOR, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.987. Support: 2047.` |
| 29 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.reserved not in {)}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, >, }}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ ^1.internal_type not in {ClassBody, File, IfStatement, MemberExpression}<br>	∧ ^1.roles in {DECLARATION} and not in {INCOMPLETE, LITERAL, OPERATOR, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.966. Support: 1549.` |
| 30 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.reserved not in {)}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.reserved not in {), ,, ;, >, }}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {ClassBody, File, IfStatement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, LITERAL, OPERATOR, SWITCH}<br>⇒ y = ␣<br>Confidence: 0.973. Support: 491.` |
| 31 | `  •••start_col ≥ 4<br>	∧ -1.diff_col ≤ 7<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.diff_col ≤ 6<br>	∧ -2.reserved not in {)}<br>	∧ -5.label not in {<newline>}<br>	∧ +1.internal_type not in {Identifier}<br>	∧ +1.reserved not in {), ,, ;, >, }}<br>	∧ +1.roles not in {COMMENT, LITERAL}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {ClassBody, File, IfStatement, MemberExpression}<br>	∧ ^1.roles not in {DECLARATION, INCOMPLETE, LITERAL, OPERATOR, SWITCH}<br>	∧ ^2.roles in {BODY}<br>⇒ y = ␣<br>Confidence: 0.985. Support: 233.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 10.290322580645162, "max_conf": 0.9997251033782959, "max_support": 10115, "min_conf": 0.9287819266319275, "min_support": 104, "num_rules": 31}}
```
</details>
