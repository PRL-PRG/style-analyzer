# Model report for file:///tmp/top-repos-quality-repos-xy6zxqcq/palelight.git HEAD 4da9b5af0f6a23d3523c9cb3038e942074a09839

### Dump

```json
{'created_at': '2021-08-21 00:06:03',
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
 'size': '17.0 kB',
 'tags': [],
 'uuid': '1213fd6d-aacd-4c25-87f8-23d017ffe5e4',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-xy6zxqcq/palelight.git 4da9b5af0f6a23d3523c9cb3038e942074a09839

# javascript
22 rules, avg.len. 8.1
## train
PPCR: 0.856271
### report
macro
{'f1-score': 0.6670219156115651,
 'precision': 0.7050709877137081,
 'recall': 0.6417416062125981,
 'support': 19231}
micro
{'f1-score': 0.9550725391295305,
 'precision': 0.9550725391295305,
 'recall': 0.9550725391295305,
 'support': 19231}
weighted
{'f1-score': 0.9501549084420137,
 'precision': 0.947325102420601,
 'recall': 0.9550725391295305,
 'support': 19231}
### report_full
macro
{'f1-score': 0.5745149710735777,
 'precision': 0.7050709877137081,
 'recall': 0.5306643102311108,
 'support': 22459}
micro
{'f1-score': 0.8811225713600384,
 'precision': 0.9550725391295305,
 'recall': 0.8178013268622824,
 'support': 22459}
weighted
{'f1-score': 0.8475755834172073,
 'precision': 0.911008304394885,
 'recall': 0.8178013268622824,
 'support': 22459}
## test
PPCR: 0.833816
### report
macro
{'f1-score': 0.5870245778325502,
 'precision': 0.584481787822305,
 'recall': 0.601436935955032,
 'support': 863}
micro
{'f1-score': 0.8493626882966396,
 'precision': 0.8493626882966396,
 'recall': 0.8493626882966396,
 'support': 863}
weighted
{'f1-score': 0.820647100153419,
 'precision': 0.7974253301929413,
 'recall': 0.8493626882966396,
 'support': 863}
### report_full
macro
{'f1-score': 0.5199269331702041,
 'precision': 0.584481787822305,
 'recall': 0.49595478819941763,
 'support': 1035}
micro
{'f1-score': 0.7723919915700737,
 'precision': 0.8493626882966396,
 'recall': 0.7082125603864734,
 'support': 1035}
weighted
{'f1-score': 0.7267218993579638,
 'precision': 0.7654766460613536,
 'recall': 0.7082125603864734,
 'support': 1035}
```

## javascript
### Summary
16 rules, avg.len. 7.4

| | |
|-|-|
|Min support|96|
|Max support|4639|
|Min confidence|0.9352570176124573|
|Max confidence|0.9971590638160706|

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
| 1 | `  ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.992. Support: 4639.` |
| 2 | `  -1.reserved = (<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 121.` |
| 3 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.971. Support: 973.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.951. Support: 844.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(}<br>	∧ -2.label not in {<space>}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved = =<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 97.` |
| 6 | `  +1.reserved = }<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.946. Support: 622.` |
| 7 | `  -1.reserved = {<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.978. Support: 575.` |
| 8 | `  -1.reserved not in {{}<br>	∧ -1.roles in {IDENTIFIER}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.935. Support: 2062.` |
| 9 | `  -1.reserved = (<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.971. Support: 985.` |
| 10 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = {<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.992. Support: 563.` |
| 11 | `  -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -5.reserved = ,<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {LITERAL} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.984. Support: 96.` |
| 12 | `  -1.diff_col ≤ 6<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ -1.length ≥ 3<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles in {EXPRESSION}<br>	∧ ^1.roles in {SCOPE} and not in {IDENTIFIER, LITERAL, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 156.` |
| 13 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {(, {}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {{, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles in {CALL} and not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.976. Support: 147.` |
| 14 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ;<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.997. Support: 176.` |
| 15 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved = ,<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.roles in {EXPRESSION} and not in {COMMENT}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 131.` |
| 16 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {IDENTIFIER}<br>	∧ +1.reserved not in {(, {, }}<br>	∧ +1.roles not in {EXPRESSION}<br>	∧ +2.reserved not in {(}<br>	∧ +2.roles not in {COMMENT, EXPRESSION}<br>	∧ ^1.internal_type not in {VariableDeclarator}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.943. Support: 854.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 7.375, "max_conf": 0.9971590638160706, "max_support": 4639, "min_conf": 0.9352570176124573, "min_support": 96, "num_rules": 16}}
```
</details>
