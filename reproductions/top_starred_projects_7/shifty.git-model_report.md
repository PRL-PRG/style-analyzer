# Model report for file:///tmp/top-repos-quality-repos-muw4dbdq/shifty.git HEAD 520ba431598d3c8e662b9347804b95ddca482134

### Dump

```json
{'created_at': '2021-08-30 18:40:33',
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
 'size': '16.9 kB',
 'tags': [],
 'uuid': '3a00a93e-68be-4c7d-a2bd-4d39c854b8ea',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-muw4dbdq/shifty.git 520ba431598d3c8e662b9347804b95ddca482134

# javascript
90 rules, avg.len. 6.1
## train
PPCR: 0.973698
### report
macro
{'f1-score': 0.7605297310578417,
 'precision': 0.8871756794197128,
 'recall': 0.701182927305883,
 'support': 12846}
micro
{'f1-score': 0.9038611240853184,
 'precision': 0.9038611240853184,
 'recall': 0.9038611240853184,
 'support': 12846}
weighted
{'f1-score': 0.8945808664967022,
 'precision': 0.9042939974286127,
 'recall': 0.9038611240853184,
 'support': 12846}
### report_full
macro
{'f1-score': 0.7313241492971415,
 'precision': 0.8871756794197128,
 'recall': 0.6677985164520542,
 'support': 13193}
micro
{'f1-score': 0.8918161219708899,
 'precision': 0.9038611240853184,
 'recall': 0.8800879254149928,
 'support': 13193}
weighted
{'f1-score': 0.8775420557502617,
 'precision': 0.9029095179815099,
 'recall': 0.8800879254149928,
 'support': 13193}
## test
PPCR: 0.747664
### report
macro
{'f1-score': 0.29128782547501764,
 'precision': 0.3391705069124424,
 'recall': 0.28208616780045354,
 'support': 80}
micro
{'f1-score': 0.65, 'precision': 0.65, 'recall': 0.65, 'support': 80}
weighted
{'f1-score': 0.6307389162561576,
 'precision': 0.6487903225806452,
 'recall': 0.65,
 'support': 80}
### report_full
macro
{'f1-score': 0.26964070135096296,
 'precision': 0.3391705069124424,
 'recall': 0.24081632653061222,
 'support': 107}
micro
{'f1-score': 0.5561497326203209,
 'precision': 0.65,
 'recall': 0.48598130841121495,
 'support': 107}
weighted
{'f1-score': 0.5154816750973128,
 'precision': 0.5791377750979801,
 'recall': 0.48598130841121495,
 'support': 107}
```

## javascript
### Summary
50 rules, avg.len. 5.8

| | |
|-|-|
|Min support|144|
|Max support|1627|
|Min confidence|0.920258641242981|
|Max confidence|0.9987980723381042|

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
| 1 | `  ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.989. Support: 1593.` |
| 2 | `  -1.reserved not in {(}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 697.` |
| 3 | `  -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 359.` |
| 4 | `  -1.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.927. Support: 388.` |
| 5 | `  -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 174.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 144.` |
| 7 | `  ^1.internal_type = MemberExpression<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1626.` |
| 8 | `  +1.reserved = )<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.930. Support: 931.` |
| 9 | `  -1.reserved = (<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LITERAL}<br>⇒ y = ∅<br>Confidence: 0.941. Support: 566.` |
| 10 | `  -1.reserved not in {(}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 416.` |
| 11 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>⇒ y = ∅<br>Confidence: 0.998. Support: 243.` |
| 12 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_offset ≤ 4<br>	∧ +1.reserved = ,<br>⇒ y = ∅<br>Confidence: 0.953. Support: 182.` |
| 13 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, >}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 185.` |
| 14 | `  •••start_col ≤ 19<br>	∧ -1.diff_col ≥ 2<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {BLOCK} and not in {FILE}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 196.` |
| 15 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {BLOCK, FILE}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.960. Support: 260.` |
| 16 | `  +1.reserved = )<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.931. Support: 941.` |
| 17 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 246.` |
| 18 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 472.` |
| 19 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_col ≤ 4<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 175.` |
| 20 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 196.` |
| 21 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, >}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 22 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.921. Support: 1056.` |
| 23 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 231.` |
| 24 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {)}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.980. Support: 269.` |
| 25 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -2.diff_col ≤ 4<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 173.` |
| 26 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, }}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 194.` |
| 27 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 186.` |
| 28 | `  -1.internal_type = Identifier<br>	∧ +1.reserved = =<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 253.` |
| 29 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved = ><br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 200.` |
| 30 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ -4.diff_offset ≤ 13<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 171.` |
| 31 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(, {}<br>	∧ +1.reserved not in {,, >}<br>	∧ +1.roles in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 183.` |
| 32 | `  -1.internal_type not in {Identifier}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.942. Support: 1067.` |
| 33 | `  -1.reserved = (<br>	∧ +1.length ≤ 11<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 888.` |
| 34 | `  -1.reserved not in {(}<br>	∧ +1.reserved = =<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 651.` |
| 35 | `  -1.reserved not in {(}<br>	∧ +1.reserved = {<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ␣<br>Confidence: 0.983. Support: 390.` |
| 36 | `  -1.reserved not in {(}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +3.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 394.` |
| 37 | `  -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 199.` |
| 38 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 149.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.986. Support: 1573.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = (<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.939. Support: 204.` |
| 41 | `  -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, ), {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,, >}<br>	∧ +1.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.974. Support: 215.` |
| 42 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.diff_offset ≥ 8<br>	∧ +1.reserved = }<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.920. Support: 232.` |
| 43 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.987. Support: 1627.` |
| 44 | `  +1.reserved = )<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 978.` |
| 45 | `  -1.roles in {IDENTIFIER}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 241.` |
| 46 | `  -1.reserved = {<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {), ,}<br>	∧ +1.roles not in {MAP}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.981. Support: 242.` |
| 47 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {,, >}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 164.` |
| 48 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.997. Support: 183.` |
| 49 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.reserved not in {=, {, }}<br>	∧ +1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ +1.length ≥ 2<br>	∧ ^1.internal_type not in {MemberExpression}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 191.` |
| 50 | `  -1.reserved = (<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {MemberExpression}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 905.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.8, "max_conf": 0.9987980723381042, "max_support": 1627, "min_conf": 0.920258641242981, "min_support": 144, "num_rules": 50}}
```
</details>
