# Model report for file:///tmp/top-repos-quality-repos-4i_imopo/clockpicker.git HEAD e6ac014b3c167281ac37cf122ab19b6967d8fae4

### Dump

```json
{'created_at': '2021-08-30 02:22:29',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.11.0-31-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.2 kB',
 'tags': [],
 'uuid': 'fb56c75d-27b7-4854-89e0-0efb37a0f91b',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-4i_imopo/clockpicker.git e6ac014b3c167281ac37cf122ab19b6967d8fae4

# javascript
60 rules, avg.len. 5.2
## train
PPCR: 0.918146
### report
macro
{'f1-score': 0.3904684870334763,
 'precision': 0.3873596975988883,
 'recall': 0.3952414036878998,
 'support': 6360}
micro
{'f1-score': 0.8734276729559748,
 'precision': 0.8734276729559748,
 'recall': 0.8734276729559748,
 'support': 6360}
weighted
{'f1-score': 0.8425813817471345,
 'precision': 0.8174812169112646,
 'recall': 0.8734276729559748,
 'support': 6360}
### report_full
macro
{'f1-score': 0.38418730529882333,
 'precision': 0.3873596975988883,
 'recall': 0.38365855121562437,
 'support': 6927}
micro
{'f1-score': 0.8361556408519605,
 'precision': 0.8734276729559748,
 'recall': 0.8019344593619171,
 'support': 6927}
weighted
{'f1-score': 0.7815497412737388,
 'precision': 0.7680477646009023,
 'recall': 0.8019344593619171,
 'support': 6927}
## test
PPCR: 0.935150
### report
macro
{'f1-score': 0.19681545029439193,
 'precision': 0.2018538560654735,
 'recall': 0.2126569962856033,
 'support': 10844}
micro
{'f1-score': 0.6565842862412394,
 'precision': 0.6565842862412394,
 'recall': 0.6565842862412394,
 'support': 10844}
weighted
{'f1-score': 0.6061528491078167,
 'precision': 0.6200730003180127,
 'recall': 0.6565842862412394,
 'support': 10844}
### report_full
macro
{'f1-score': 0.19399476851173775,
 'precision': 0.2018538560654735,
 'recall': 0.208923797391422,
 'support': 11596}
micro
{'f1-score': 0.6345811051693406,
 'precision': 0.6565842862412394,
 'recall': 0.614004829251466,
 'support': 11596}
weighted
{'f1-score': 0.5720048207233304,
 'precision': 0.5972451924122373,
 'recall': 0.614004829251466,
 'support': 11596}
```

## javascript
### Summary
40 rules, avg.len. 5.0

| | |
|-|-|
|Min support|134|
|Max support|1991|
|Min confidence|0.9248927235603333|
|Max confidence|0.9990347623825073|

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
                     'min_samples_split': 182,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>⇒ y = '<br>Confidence: 0.999. Support: 354.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.977. Support: 149.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.961. Support: 243.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {EXPRESSION} and not in {IF, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.958. Support: 1889.` |
| 5 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.946. Support: 1163.` |
| 6 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 292.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 179.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type = StringLiteral<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 152.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 195.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.960. Support: 781.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 298.` |
| 12 | `  -1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.998. Support: 298.` |
| 13 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 158.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 208.` |
| 15 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 143.` |
| 16 | `  -1.roles not in {STRING}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.946. Support: 140.` |
| 17 | `  -1.roles in {STRING}<br>⇒ y = '<br>Confidence: 0.999. Support: 339.` |
| 18 | `  -1.reserved = (<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.985. Support: 170.` |
| 19 | `  -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ -2.label not in {<space>}<br>	∧ +1.roles in {LITERAL}<br>⇒ y = '<br>Confidence: 0.997. Support: 163.` |
| 20 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type = VariableDeclarator<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.925. Support: 233.` |
| 21 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.980. Support: 1340.` |
| 22 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = .<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 518.` |
| 23 | `  -1.diff_offset ≤ 2<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.roles not in {SCOPE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 303.` |
| 24 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 289.` |
| 25 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.957. Support: 219.` |
| 26 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.roles not in {LITERAL}<br>	∧ ^1.roles in {EXPRESSION} and not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.952. Support: 1955.` |
| 27 | `  -1.reserved not in {;}<br>	∧ -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 743.` |
| 28 | `  -1.reserved not in {;}<br>	∧ -1.roles not in {EXPRESSION, STRING}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.roles not in {VARIABLE}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 291.` |
| 29 | `  -1.internal_type = StringLiteral<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = '<br>Confidence: 0.998. Support: 280.` |
| 30 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.932. Support: 259.` |
| 31 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {EXPRESSION} and not in {IF, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.956. Support: 1991.` |
| 32 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 180.` |
| 33 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.997. Support: 175.` |
| 34 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.931. Support: 282.` |
| 35 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type not in {VariableDeclaration}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 181.` |
| 36 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.981. Support: 1180.` |
| 37 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 192.` |
| 38 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {EXPRESSION} and not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.983. Support: 208.` |
| 39 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 134.` |
| 40 | `  -1.internal_type not in {StringLiteral}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.roles not in {OPERATOR, QUALIFIED}<br>	∧ ^2.internal_type = VariableDeclaration<br>⇒ y = ␣<br>Confidence: 0.953. Support: 139.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 4.95, "max_conf": 0.9990347623825073, "max_support": 1991, "min_conf": 0.9248927235603333, "min_support": 134, "num_rules": 40}}
```
</details>
