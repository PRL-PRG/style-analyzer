# Model report for file:///tmp/top-repos-quality-repos-9alsofqb/avimehenwal.in.git HEAD c8dd7a458c1882832238980000b145ae1b6d32a9

### Dump

```json
{'created_at': '2021-08-21 04:57:06',
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
 'size': '20.9 kB',
 'tags': [],
 'uuid': '8f4b31ec-d9f1-4b14-87ca-808317959d8a',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-9alsofqb/avimehenwal.in.git c8dd7a458c1882832238980000b145ae1b6d32a9

# javascript
27 rules, avg.len. 7.0
## train
PPCR: 0.874825
### report
macro
{'f1-score': 0.5131105197719971,
 'precision': 0.5288111507299181,
 'recall': 0.5047149135543313,
 'support': 38648}
micro
{'f1-score': 0.9550300144897537,
 'precision': 0.9550300144897537,
 'recall': 0.9550300144897537,
 'support': 38648}
weighted
{'f1-score': 0.9443949769890915,
 'precision': 0.9351534947316602,
 'recall': 0.9550300144897537,
 'support': 38648}
### report_full
macro
{'f1-score': 0.42003848649979125,
 'precision': 0.5288111507299181,
 'recall': 0.38346817528560045,
 'support': 44178}
micro
{'f1-score': 0.891266027575882,
 'precision': 0.9550300144897537,
 'recall': 0.8354837249309611,
 'support': 44178}
weighted
{'f1-score': 0.8496884495011752,
 'precision': 0.8806818955478235,
 'recall': 0.8354837249309611,
 'support': 44178}
## test
PPCR: 0.682413
### report
macro
{'f1-score': 0.25458368510016377,
 'precision': 0.24162489560442188,
 'recall': 0.3281843405214104,
 'support': 3496}
micro
{'f1-score': 0.7743135011441648,
 'precision': 0.7743135011441648,
 'recall': 0.7743135011441648,
 'support': 3496}
weighted
{'f1-score': 0.7558632867396866,
 'precision': 0.7523949828143235,
 'recall': 0.7743135011441648,
 'support': 3496}
### report_full
macro
{'f1-score': 0.23003986103943888,
 'precision': 0.24162489560442188,
 'recall': 0.27559969388530375,
 'support': 5123}
micro
{'f1-score': 0.6281471168348997,
 'precision': 0.7743135011441648,
 'recall': 0.5284013273472574,
 'support': 5123}
weighted
{'f1-score': 0.5296312733140462,
 'precision': 0.5421745298741442,
 'recall': 0.5284013273472574,
 'support': 5123}
```

## javascript
### Summary
17 rules, avg.len. 6.8

| | |
|-|-|
|Min support|90|
|Max support|6899|
|Min confidence|0.9215182662010193|
|Max confidence|0.9991150498390198|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 6899.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 240.` |
| 3 | `  -1.reserved not in {(}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.954. Support: 226.` |
| 4 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {STRING}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.967. Support: 4439.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 565.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ -4.label not in {<newline>}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.957. Support: 5135.` |
| 7 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type = StringLiteral<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.932. Support: 302.` |
| 8 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 2173.` |
| 9 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 834.` |
| 10 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label in {<-space>}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻␣⁻␣⁻<br>Confidence: 0.939. Support: 90.` |
| 11 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 561.` |
| 12 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.978. Support: 473.` |
| 13 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.997. Support: 148.` |
| 14 | `  -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = else<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.926. Support: 128.` |
| 15 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.998. Support: 213.` |
| 16 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ,, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.995. Support: 102.` |
| 17 | `  -1.diff_col ≤ 8<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ +1.reserved not in {), ,, ;, }}<br>	∧ +1.roles not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclaration}<br>	∧ ^1.roles not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.922. Support: 5256.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 6.764705882352941, "max_conf": 0.9991150498390198, "max_support": 6899, "min_conf": 0.9215182662010193, "min_support": 90, "num_rules": 17}}
```
</details>
