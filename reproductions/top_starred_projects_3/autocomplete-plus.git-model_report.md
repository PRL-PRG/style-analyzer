# Model report for file:///tmp/top-repos-quality-repos-btoso_kg/autocomplete-plus.git HEAD d2a02b1fb7d228b2ea00065d6fe19f96f5731861

### Dump

```json
{'created_at': '2021-08-29 22:02:21',
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
 'size': '16.5 kB',
 'tags': [],
 'uuid': '66fd7e40-144d-4541-9537-70b3caef56e0',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-btoso_kg/autocomplete-plus.git d2a02b1fb7d228b2ea00065d6fe19f96f5731861

# javascript
21 rules, avg.len. 7.2
## train
PPCR: 0.898931
### report
macro
{'f1-score': 0.913214653930041,
 'precision': 0.9507937651149737,
 'recall': 0.8907750693285426,
 'support': 29671}
micro
{'f1-score': 0.9757675845101277,
 'precision': 0.9757675845101277,
 'recall': 0.9757675845101277,
 'support': 29671}
weighted
{'f1-score': 0.9751922616724817,
 'precision': 0.9761909594539557,
 'recall': 0.9757675845101277,
 'support': 29671}
### report_full
macro
{'f1-score': 0.7561892534624641,
 'precision': 0.9507937651149737,
 'recall': 0.6979560877357783,
 'support': 33007}
micro
{'f1-score': 0.9238329238329237,
 'precision': 0.9757675845101277,
 'recall': 0.8771472717908323,
 'support': 33007}
weighted
{'f1-score': 0.9045553808856626,
 'precision': 0.9752500010538658,
 'recall': 0.8771472717908323,
 'support': 33007}
## test
PPCR: 0.894662
### report
macro
{'f1-score': 0.9482115309719267,
 'precision': 0.9786855766534158,
 'recall': 0.9275747227844465,
 'support': 14413}
micro
{'f1-score': 0.9941719281204469,
 'precision': 0.9941719281204469,
 'recall': 0.9941719281204469,
 'support': 14413}
weighted
{'f1-score': 0.9940313823151691,
 'precision': 0.9941310732909125,
 'recall': 0.9941719281204469,
 'support': 14413}
### report_full
macro
{'f1-score': 0.7509387636050997,
 'precision': 0.9786855766534158,
 'recall': 0.7075858669903216,
 'support': 16110}
micro
{'f1-score': 0.9388985355305836,
 'precision': 0.9941719281204469,
 'recall': 0.889447548106766,
 'support': 16110}
weighted
{'f1-score': 0.9135733559999588,
 'precision': 0.9915218184208258,
 'recall': 0.889447548106766,
 'support': 16110}
```

## javascript
### Summary
19 rules, avg.len. 7.4

| | |
|-|-|
|Min support|117|
|Max support|6578|
|Min confidence|0.9288991093635559|
|Max confidence|0.999475359916687|

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
                     'min_samples_split': 185,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.996. Support: 132.` |
| 2 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 6578.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 953.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.997. Support: 501.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = (<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 2313.` |
| 6 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.999. Support: 822.` |
| 7 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≥ 2<br>	∧ ^1.roles in {BINARY} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 248.` |
| 8 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION, NAME}<br>	∧ +1.reserved not in {=}<br>	∧ +1.length ≤ 1<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 217.` |
| 9 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles in {EXPRESSION} and not in {NAME}<br>	∧ +1.reserved not in {=, }}<br>	∧ +1.length ≤ 1<br>	∧ ^1.internal_type not in {BinaryExpression}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 3746.` |
| 10 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {QUALIFIED}<br>⇒ y = '<br>Confidence: 0.999. Support: 453.` |
| 11 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≥ 2<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {BODY} and not in {QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.991. Support: 527.` |
| 12 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {BODY} and not in {QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.950. Support: 307.` |
| 13 | `  •••start_col ≤ 9<br>	∧ -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -1.length ≤ 1<br>	∧ -2.diff_offset ≤ 5<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.roles in {BODY} and not in {QUALIFIED}<br>⇒ y = ⏎⏎<br>Confidence: 0.929. Support: 218.` |
| 14 | `  -1.internal_type = CommentLine<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {BODY, QUALIFIED}<br>⇒ y = ⏎<br>Confidence: 0.979. Support: 211.` |
| 15 | `  -1.internal_type not in {CommentLine, StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.internal_type = File<br>	∧ ^1.roles not in {BODY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 199.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ><br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BODY, QUALIFIED}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 248.` |
| 17 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BODY}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 156.` |
| 18 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BODY, LITERAL}<br>⇒ y = ∅<br>Confidence: 0.936. Support: 117.` |
| 19 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, [, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,, >, }}<br>	∧ ^1.internal_type not in {File}<br>	∧ ^1.roles not in {BODY, LITERAL, QUALIFIED}<br>⇒ y = ␣<br>Confidence: 0.933. Support: 4408.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.368421052631579, "max_conf": 0.999475359916687, "max_support": 6578, "min_conf": 0.9288991093635559, "min_support": 117, "num_rules": 19}}
```
</details>
